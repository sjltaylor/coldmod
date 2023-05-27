use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

use coldmod_msg::proto::Trace;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
use flexbuffers;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use async_channel::Receiver;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone)]
// struct SocketContext {
//     receiver: Receiver<Trace>,
// }

#[tonic::async_trait]
pub trait DispatchContext: Clone + Send + Sync + 'static {
    fn receiver(&self) -> Receiver<Trace>;
}

pub async fn server<Dispatch: DispatchContext>(dispatch: Dispatch) {
    // build our application with some routes
    let app: _ = Router::new()
        .route("/ws", get(ws_handler::<Dispatch>).with_state(dispatch))
        .route("/", get(|| async { "Hello, World!" }))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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
async fn ws_handler<Dispatch: DispatchContext>(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(dispatch): State<Dispatch>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, dispatch))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket<Dispatch: DispatchContext>(
    socket: WebSocket,
    who: SocketAddr,
    dispatch: Dispatch,
) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        while let Ok(trace) = dispatch.receiver().recv().await {
            let we = coldmod_msg::web::Event::DaemonEmitsSourceData {};

            let mut flexbuffers_serializer = flexbuffers::FlexbufferSerializer::new();
            we.serialize(&mut flexbuffers_serializer).unwrap();

            // In case of any websocket error, we exit.
            if sender
                .send(Message::Binary(flexbuffers_serializer.take_buffer()))
                .await
                .is_err()
            {
                eprintln!("Could not send message, bailing out");
                return;
            }
        }

        println!("source socket closed/errored, sending close to client {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {}, probably it is ok?", e);
        };
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut count = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            count += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        count
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => println!("messages forwarding finished, closing {}", who),
                Err(a) => println!("Error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {} messages", b),
                Err(b) => println!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {} destroyed", who);
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            let reader = flexbuffers::Reader::get_root(d.as_slice()).unwrap();
            let event = coldmod_msg::web::Event::deserialize(reader).unwrap();
            println!("recv: {} sent a {} byte event: {:?}", who, d.len(), event);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {} sent pong with {:?}", who, v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}
