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

use crate::dispatch::Dispatch;

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
    ws.on_upgrade(move |socket| serve_socket(socket, addr, dispatch))
}

async fn serve_socket(socket: WebSocket, who: SocketAddr, dispatch: Dispatch) {
    let (ws_sender, ws_receiver) = socket.split();
    let (response_sender, response_receiver) = tokio::sync::mpsc::channel(65536);
    let initialization_msg_sender = response_sender.clone();

    let dispatch_msgs = dispatch.receiver();
    let mut send_task = tokio::spawn(async move {
        dispatch_to_websocket(dispatch_msgs, response_receiver, ws_sender).await;
    });

    let receiver_dispatch = dispatch.clone();
    let mut recv_task = tokio::spawn(async move {
        websocket_to_dispatch(ws_receiver, receiver_dispatch, response_sender).await;
    });

    match dispatch
        .handle(coldmod_msg::web::Msg::AppSocketConnected)
        .await
    {
        Ok(msgs) => {
            for msg in msgs {
                match initialization_msg_sender.send(msg).await {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("error sending initialization message {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!("error handling AppSocketConnected {:?}", e);
        }
    }

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => tracing::trace!("messages forwarding finished, closing {}", who),
                Err(a) => tracing::error!("error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_) =>  tracing::trace!("message listening finished {}", who),
                Err(b) =>  tracing::error!("error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }
}

async fn websocket_to_dispatch(
    mut ws_receiver: SplitStream<WebSocket>,
    dispatch: Dispatch,
    response_sender: tokio::sync::mpsc::Sender<coldmod_msg::web::Msg>,
) {
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Binary(payload) => match flexbuffers::from_slice(&payload) {
                Ok(msg) => {
                    tracing::info!("websocket -> dispatch: {msg}");
                    match dispatch.handle(msg).await {
                        Ok(msgs) => {
                            for msg in msgs {
                                match response_sender.send(msg).await {
                                    Ok(_) => tracing::info!("response relayed"),
                                    Err(e) => tracing::error!("error relaying response: {}", e),
                                }
                            }
                        }
                        Err(e) => tracing::error!("error dispatching message: {:?}", e),
                    }
                }
                Err(e) => tracing::error!("error unmarshalling message: {:?}", e),
            },
            Message::Close(_) => {
                tracing::info!("closing connection");
                return;
            }
            _ => {
                tracing::debug!("ignoring message: {:?}", msg);
            }
        }
    }
}

async fn dispatch_to_websocket(
    mut dispatch_msgs: tokio::sync::broadcast::Receiver<coldmod_msg::web::Msg>,
    mut response_msgs: tokio::sync::mpsc::Receiver<coldmod_msg::web::Msg>,
    mut ws_sender: SplitSink<WebSocket, Message>,
) {
    loop {
        let msg = tokio::select! {
            dispatch_r = dispatch_msgs.recv() => {
               match dispatch_r {
                    Ok(msg) => msg,
                    Err(e) => {
                        tracing::error!("dispatch receiver error: {:?}", e);
                        break;
                    },
               }
            },
            response_r = response_msgs.recv() => {
                match response_r {
                    Some(msg) => msg,
                    None => {
                        tracing::error!("response recveiver closed");
                        break;
                    },
                }
            }
        };

        let msg = match msg {
            coldmod_msg::web::Msg::SourceDataAvailable(_) => Some(msg),
            coldmod_msg::web::Msg::TracingStatsAvailable(_) => Some(msg),
            _ => None,
        };

        if msg.is_none() {
            continue;
        }

        let msg = msg.unwrap();

        tracing::info!("dispatch -> websocket: {msg}");

        match flexbuffers::to_vec(msg) {
            Ok(payload) => {
                if let Err(e) = ws_sender.send(Message::Binary(payload)).await {
                    tracing::error!("could not send message: {:?}", e);
                    return;
                } else {
                    tracing::trace!("sent message");
                }
            }
            Err(e) => tracing::error!("error marshalling message: {:?}", e),
        }
    }
}
