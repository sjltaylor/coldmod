use anyhow::anyhow;
use async_trait::async_trait;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use coldmod_msg::web::Msg;

use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, TraceLayer},
    validate_request::ValidateRequestHeaderLayer,
};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use flexbuffers;

use crate::dispatch::{self, Dispatch};

pub async fn server(dispatch: Dispatch) {
    let web_host = dispatch.web_host();
    let api_key = dispatch.api_key();
    let tls = dispatch.tls();

    let serve_dir = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

    // build our application with some routes
    let mut app = Router::new()
        .route("/ws/connect/:key", get(ws_handler).with_state(dispatch))
        .nest_service("/", serve_dir)
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    if let Some(api_key) = api_key {
        let auth_service = ValidateRequestHeaderLayer::basic("coldmod", api_key.as_str());
        app = app.layer(auth_service);
    }

    if let Some((cert, key)) = tls {
        // configure certificate and private key used by https
        let config = RustlsConfig::from_pem_file(cert, key).await.unwrap();
        tracing::info!("starting https server on {}", web_host);
        axum_server::bind_rustls(web_host, config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    } else {
        tracing::info!("starting http server on {}", web_host);
        axum::Server::bind(&web_host)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(key): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(dispatch): State<Dispatch>,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    tracing::trace!("websocket upgrade request from {}", addr);

    // let key = format!("{:?}", ws_key);
    println!("{:?}", key);

    ws.on_upgrade(move |socket| serve_socket(socket, dispatch, key))
}

#[derive(Clone)]
struct WebSocketAdapter {
    socket: Arc<Mutex<WebSocket>>,
    key: String,
}

#[async_trait]
impl dispatch::WebSocket for WebSocketAdapter {
    async fn send(&mut self, msg: &Msg) -> Result<(), anyhow::Error> {
        let payload = flexbuffers::to_vec(msg)?;
        let mut lock = self.socket.as_ref().lock().await;
        lock.send(Message::Binary(payload)).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Option<Result<Msg, anyhow::Error>> {
        let mut lock = self.socket.as_ref().lock().await;
        match lock.recv().await {
            None | Some(Ok(Message::Close(_))) => {
                tracing::info!("socket closed");
                return None;
            }
            Some(Ok(Message::Binary(payload))) => match flexbuffers::from_slice(&payload) {
                Ok(msg) => match msg {
                    Msg::RouteModCommand(c) => {
                        Some(Ok(Msg::RouteModCommandTo((c, self.key.clone()))))
                    }
                    _ => Some(Ok(msg)),
                },
                Err(e) => Some(Err(e.into())),
            },
            Some(Ok(_)) => Some(Err(anyhow!("unexpected message type"))),
            Some(Err(e)) => Some(Err(e.into())),
        }
    }

    fn key(&self) -> String {
        self.key.clone()
    }
}

async fn serve_socket(socket: WebSocket, dispatch: Dispatch, key: String) {
    let ws = WebSocketAdapter {
        socket: Arc::new(Mutex::new(socket)),
        key,
    };
    dispatch.serve_socket(ws).await;
}
