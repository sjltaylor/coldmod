use crate::events::{AppEvent, WebSocketEventType};
use crate::Dispatch;
use leptos::*;
use wasm_bindgen::prelude::*;

use web_sys::*;

pub fn start(dispatch: Dispatch) {
    let ws = WebSocket::new("ws://localhost:3333/ws").expect("to create websocket");
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    relay_upstream(dispatch.clone(), &ws);

    {
        // onopen
        let onopen_dispatch = dispatch.clone();
        let onopen_callback = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            let app_event = AppEvent::WebSocketClientEvent(WebSocketEventType::Open);
            onopen_dispatch.emit(app_event);
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    {
        let onclose_dispatch = dispatch.clone();
        let onclose_callback = Closure::<dyn FnMut(_)>::new(move |close_event: CloseEvent| {
            let app_event =
                AppEvent::WebSocketClientEvent(WebSocketEventType::Close(close_event.clone()));
            onclose_dispatch.emit(app_event);
        });
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
    }

    {
        // // onmessage
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            relay_downstream(e, dispatch.clone());
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

fn relay_upstream(dispatch: Dispatch, ws: &WebSocket) {
    // TODO: remove this fn if the app never emits events
    let _ws = ws.clone();

    dispatch.on_app_event(move |app_event| match app_event {
        AppEvent::ColdmodMsg(msg_event) => match msg_event {
            _ => {}
        },
        _ => {}
    });
}

fn relay_downstream(e: MessageEvent, dispatch: Dispatch) {
    if let Ok(data) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
        let buffer = js_sys::Uint8Array::new(&data).to_vec();
        let msg = match flexbuffers::from_slice(&buffer) {
            Ok(msg) => msg,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        log!("recv: {:?}", msg);

        dispatch.emit(AppEvent::ColdmodMsg(msg));

        return;
    }

    warn!("unexpected message: {:?}", e.data());
}
