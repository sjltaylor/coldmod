


use leptos::*;
use serde::{Deserialize};

use wasm_bindgen::prelude::*;

use web_sys::*;

pub fn start(s: async_channel::Sender<String>) {
    // Connect to an echo server
    let ws = WebSocket::new("ws://localhost:3000/ws").expect("to create websocket");
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e| onmessage(e, &s));
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onclose_callback = Closure::<dyn FnMut(_)>::new(onclose);
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();

    let ws_clone = ws.clone();
    let onopen_callback = Closure::<dyn FnMut(_)>::new(move |e| onopen(&ws_clone, e));
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(onerror);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
}

fn onopen(_: &WebSocket, _e: Event) {
    //todo!("onopen {:?}", e);
}

fn onmessage(e: MessageEvent, snd: &async_channel::Sender<String>) {
    // Handle difference Text/Binary,...
    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
        let s = format!("message event, received Text: {:?}", txt);
        match snd.try_send(s) {
            Ok(_) => {}
            Err(e) => {
                log!("Error sending message: {:?}", e);
            }
        }

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

fn onerror(e: ErrorEvent) {
    todo!("onerror {:?}", e);
}

fn onclose(e: CloseEvent) {
    todo!("onclose {:?}", e);
}
