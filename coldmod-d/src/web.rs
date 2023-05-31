use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

use futures_util::stream::{SplitSink, SplitStream};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use flexbuffers;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use serde::{Deserialize, Serialize};

use crate::dispatch::WebDispatch;

pub async fn server<Dispatch: WebDispatch>(dispatch: Dispatch) {
    // build our application with some routes
    let app = Router::new()
        .route("/ws", get(ws_handler::<Dispatch>).with_state(dispatch))
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
async fn ws_handler<Dispatch: WebDispatch>(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(dispatch): State<Dispatch>,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| serve_socket(socket, addr, dispatch))
}

async fn serve_socket<Dispatch: WebDispatch>(
    socket: WebSocket,
    who: SocketAddr,
    dispatch: Dispatch,
) {
    let (ws_sender, ws_receiver) = socket.split();

    let sender_dispatch = dispatch.clone();
    let mut send_task = tokio::spawn(async move {
        dispatch_to_websocket(sender_dispatch, ws_sender).await;
    });

    let mut recv_task = tokio::spawn(async move {
        websocket_to_dispatch(ws_receiver, dispatch).await;
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => tracing::trace!("websocket: messages forwarding finished, closing {}", who),
                Err(a) => tracing::error!("websocket: error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_) =>  tracing::trace!("websocket: message listening finished {}", who),
                Err(b) =>  tracing::error!("websocket: error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }
}

async fn websocket_to_dispatch<Dispatch: WebDispatch>(
    mut ws_receiver: SplitStream<WebSocket>,
    dispatch: Dispatch,
) {
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Binary(payload) => match unmarshall(&payload) {
                Ok(event) => match dispatch.emit(event).await {
                    Ok(_) => {}
                    Err(e) => tracing::error!("websocket: error dispatching message: {:?}", e),
                },
                Err(e) => tracing::error!("websocket: error unmarshalling message: {:?}", e),
            },
            _ => {
                tracing::trace!("websocket: ignoring message: {:?}", msg);
            }
        }
    }
}

async fn dispatch_to_websocket<Dispatch: WebDispatch>(
    mut dispatch: Dispatch,
    mut ws_sender: SplitSink<WebSocket, Message>,
) {
    while let Ok(event) = dispatch.receive().await {
        let event = match event {
            coldmod_msg::web::Event::SourceDataAvailable(_) => Some(event),
            _ => None,
        };

        println!("forwarding event {:?}", event);

        if event.is_none() {
            continue;
        }

        let event = event.unwrap();

        match marshall(event) {
            Ok(payload) => {
                if let Err(e) = ws_sender.send(Message::Binary(payload)).await {
                    tracing::error!("websocket: could not send message: {:?}", e);
                    return;
                }
            }
            Err(e) => tracing::error!("websocket: error marshalling message: {:?}", e),
        }
    }
}

fn unmarshall(payload: &Vec<u8>) -> Result<coldmod_msg::web::Event, anyhow::Error> {
    let reader = flexbuffers::Reader::get_root(payload.as_slice())?;
    let event = coldmod_msg::web::Event::deserialize(reader)?;
    Ok(event)
}

fn marshall(event: coldmod_msg::web::Event) -> Result<Vec<u8>, anyhow::Error> {
    let mut flexbuffers_serializer = flexbuffers::FlexbufferSerializer::new();
    event.serialize(&mut flexbuffers_serializer)?;
    Ok(flexbuffers_serializer.take_buffer())
}
