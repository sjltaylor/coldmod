use crate::events::{AppEvent, WebSocketEventType};
use crate::Dispatch;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use web_sys::*;

pub fn start(dispatch: &Dispatch) {
    let ws = WebSocket::new("ws://localhost:3333/ws").expect("to create websocket");
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    spawn_upstream_relay(dispatch, &ws);

    {
        // onopen
        let onopen_dispatch = dispatch.clone();
        let onopen_callback = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            if let Err(e) =
                onopen_dispatch.send(AppEvent::WebSocketClientEvent(WebSocketEventType::Open))
            {
                error!("to send websocket open event: {:?}", e);
            }
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    {
        // onclose
        let onclose_dispatch = dispatch.clone();
        let onclose_callback = Closure::<dyn FnMut(_)>::new(move |close_event: CloseEvent| {
            if let Err(e) = onclose_dispatch.send(AppEvent::WebSocketClientEvent(
                WebSocketEventType::Close(close_event.clone()),
            )) {
                error!("websocket closed: {:?}", close_event);
                error!(
                    "Error dispatching websocket closed event on dispatcher: {:?}",
                    e
                );
            }
        });
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
    }

    {
        // onmessage
        let onmessage_dispatch = dispatch.clone();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            relay_downstream(e, &onmessage_dispatch);
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }

    {
        // onerror
        let onerror_callback =
            Closure::<dyn FnMut(_)>::new(|_: ErrorEvent| error!("websocket error"));
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }
}

fn spawn_upstream_relay(dispatch: &Dispatch, ws: &WebSocket) {
    let receiver_dispatch = dispatch.clone();
    let ws = ws.clone();
    leptos::spawn_local(async move {
        log!("ws starting upstream relay");
        let mut queue = Vec::<coldmod_msg::web::Event>::new();
        while let Ok(event) = receiver_dispatch.receive().await {
            log!("ws event received {:?}", event);
            match event {
                AppEvent::ColdmodMsg(event) => {
                    if ws.ready_state() != WebSocket::OPEN {
                        log!("ws queueing event for relay");
                        queue.push(event);
                        continue;
                    }
                    relay_message(&event, &ws);
                }
                AppEvent::WebSocketClientEvent(wse) => {
                    log!("got websocket client event {:?}", wse);
                    match wse {
                        WebSocketEventType::Close(_) => break,
                        WebSocketEventType::Open => {
                            for event in queue.drain(..) {
                                relay_message(&event, &ws);
                            }
                        }
                    }
                }
            }
        }
    });
}

fn relay_message(event: &coldmod_msg::web::Event, ws: &WebSocket) {
    log!("ws forwarding coldmod msg event {:?}", event);
    let mut flexbuffers_serializer = flexbuffers::FlexbufferSerializer::new();
    event.serialize(&mut flexbuffers_serializer).unwrap();

    if let Err(err) = ws.send_with_u8_array(flexbuffers_serializer.take_buffer().as_slice()) {
        error!("Error sending message: {:?}", err);
    }
}

fn relay_downstream(e: MessageEvent, dispatch: &Dispatch) {
    if let Ok(data) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
        let buffer = js_sys::Uint8Array::new(&data).to_vec();
        let r = flexbuffers::Reader::get_root(buffer.as_slice()).unwrap();
        let e = coldmod_msg::web::Event::deserialize(r).unwrap();

        match e {
            coldmod_msg::web::Event::SourceDataAvailable(_) => {
                if let Err(e) = dispatch.send(AppEvent::ColdmodMsg(e)) {
                    error!("Error dispatching websocket message on dispatcher: {:?}", e);
                }
            }
            _ => {}
        }

        return;
    }

    warn!("unexpected message: {:?}", e.data());
}
