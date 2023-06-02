use coldmod_msg::web;

use futures_util::{SinkExt, StreamExt};

use tokio::sync::oneshot::Receiver;
// we will use tungstenite for websocket client impl (same library as what axum is using)
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const COLDMOD_D_URL: &str = "ws://127.0.0.1:3333/ws";

/*

* create two clients
* one sends a request for data
* only _that_ client should received an event
* a cli sends a source scan
* both clients should receive an event

*/

#[tokio::test]
async fn test_source_data_dispatch() {
    // https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/client.rs

    let (timeout1, cancel1) = tokio::sync::oneshot::channel();
    let events1 = vec![web::Msg::RequestSourceData];
    let client1 = tokio::spawn(spawn_client(1, events1, cancel1));

    let (timeout2, cancel2) = tokio::sync::oneshot::channel();
    let events2 = vec![];
    let client2 = tokio::spawn(spawn_client(2, events2, cancel2));

    let timeouts = tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        timeout1.send(()).expect("error sending cancel");
        timeout2.send(()).expect("error sending cancel");
    });

    match tokio::join!(client1, client2, timeouts) {
        (Ok(events1), Ok(events2), _) => {
            // only the client that sent the event should get a reply
            assert_eq!(
                events1.len(),
                1,
                "the first client should have received an event"
            );
            match events1[0] {
                web::Msg::SourceDataAvailable(_) => {}
                _ => assert!(false, "wrong event received"),
            }
            assert_eq!(
                events2.len(),
                0,
                "the second client should not have received an event"
            );
        }
        _ => panic!("error running test"),
    }
}

async fn spawn_client(who: usize, msgs: Vec<web::Msg>, mut cancel: Receiver<()>) -> Vec<web::Msg> {
    let mut received_messages = vec![];

    let ws_stream = match connect_async(COLDMOD_D_URL).await {
        Ok((stream, _)) => stream,
        Err(e) => {
            panic!("WebSocket handshake for client {who} failed with {e}!");
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    let send_task = tokio::spawn(async move {
        for msg in msgs.iter() {
            let payload = flexbuffers::to_vec(msg).unwrap();
            if let Err(e) = sender.send(Message::Binary(payload)).await {
                panic!("error sending message: {:?}", e);
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut cancel => {
                    break;
                }
                rcv = receiver.next() => {
                    if let Some(Ok(msg)) = rcv {
                        match msg {
                            Message::Binary(payload) => {
                                let msg = flexbuffers::from_slice(payload.as_slice())
                                    .expect("error deserializing message");
                                received_messages.push(msg);
                            }
                            Message::Close(_) => {
                                break;
                            }
                            _ => {
                                unreachable!("This is never supposed to happen. {}", msg)
                            }
                        }
                    }

                }
            }
        }
        received_messages
    });

    send_task.await.expect("error sending messages");
    return recv_task.await.unwrap();
}
