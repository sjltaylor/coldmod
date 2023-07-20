use anyhow::anyhow;
use async_trait::async_trait;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use coldmod_msg::web::Msg;

use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use flexbuffers;

use crate::dispatch::{self, Dispatch};

pub async fn server(dispatch: Dispatch) {
    // build our application with some routes
    let app = Router::new()
        .route("/ws", get(ws_handler).with_state(dispatch))
        .route("/", get(|| async { "Hello, World!" }))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(dispatch): State<Dispatch>,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    tracing::info!("websocket upgrade request from {}", addr);
    ws.on_upgrade(move |socket| serve_socket(socket, dispatch))
}

struct WebSocketAdapter {
    socket: WebSocket,
}

#[async_trait]
impl dispatch::WebSocket for WebSocketAdapter {
    async fn send(&mut self, msg: &Msg) -> Result<(), anyhow::Error> {
        let payload = flexbuffers::to_vec(msg)?;
        self.socket.send(Message::Binary(payload)).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Option<Result<Msg, anyhow::Error>> {
        match self.socket.recv().await {
            None | Some(Ok(Message::Close(_))) => {
                tracing::info!("socket closed");
                return None;
            }
            Some(Ok(Message::Binary(payload))) => match flexbuffers::from_slice(&payload) {
                Ok(msg) => Some(Ok(msg)),
                Err(e) => Some(Err(e.into())),
            },
            Some(Ok(_)) => Some(Err(anyhow!("unexpected message type"))),
            Some(Err(e)) => Some(Err(e.into())),
        }
    }
}

async fn serve_socket(socket: WebSocket, dispatch: Dispatch) {
    let ws = WebSocketAdapter { socket };
    dispatch.serve_socket(ws).await;
}
