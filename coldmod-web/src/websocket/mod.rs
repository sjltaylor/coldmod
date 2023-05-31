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

    {
        // onopen
        let ws_clone = ws.clone();
        let receiver = dispatch.channel.1.clone();
        let onopen_callback = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            spawn_upstream_relay(&receiver, &ws_clone);
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    {
        // onclose
        let sender = dispatch.channel.0.clone();
        let onclose_callback = Closure::<dyn FnMut(_)>::new(move |close_event: CloseEvent| {
            if let Err(e) = sender.try_send(AppEvent::WebSocketClientEvent(
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
        let sender = dispatch.channel.0.clone();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            relay_downstream(e, &sender);
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

fn spawn_upstream_relay(receiver: &async_channel::Receiver<AppEvent>, ws: &WebSocket) {
    let receiver = receiver.clone();
    let ws = ws.clone();
    leptos::spawn_local(async move {
        while let Ok(event) = receiver.recv().await {
            match event {
                AppEvent::ColdmodMsg(event) => {
                    log!("todo: emit HydrateSourceView");
                    let mut flexbuffers_serializer = flexbuffers::FlexbufferSerializer::new();
                    event.serialize(&mut flexbuffers_serializer).unwrap();
                    if let Err(err) =
                        ws.send_with_u8_array(flexbuffers_serializer.take_buffer().as_slice())
                    {
                        error!("Error sending message: {:?}", err);
                    }
                }
                AppEvent::WebSocketClientEvent(wse) => match wse {
                    WebSocketEventType::Close(_) => break,
                },
            }
        }
    });
}

fn relay_downstream(e: MessageEvent, _: &async_channel::Sender<AppEvent>) {
    // Handle difference Text/Binary,...
    if let Ok(_txt) = e.data().dyn_into::<js_sys::JsString>() {
        // let s = format!("message event, received Text: {:?}", txt);
        // match snd.try_send(s) {
        //     Ok(_) => {}
        //     Err(e) => {
        //         log!("Error sending message: {:?}", e);
        //     }
        // }

        return;
    }

    if let Ok(data) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
        let buffer = js_sys::Uint8Array::new(&data).to_vec();
        let r = flexbuffers::Reader::get_root(buffer.as_slice()).unwrap();
        let e = coldmod_msg::web::Event::deserialize(r).unwrap();

        log!("message event, received bytes: {:?}", e);
        // match snd.try_send(s) {
        //     Ok(_) => {}
        //     Err(e) => {
        //         log!("Error sending message: {:?}", e);
        //     }
        // }

        return;
    }

    warn!("unexpected message: {:?}", e.data());
}
