use coldmod_msg::web::Msg;
use leptos::*;
use wasm_bindgen::prelude::*;

use web_sys::*;

#[derive(Clone)]
pub struct Sender {
    ws: WebSocket,
}

impl Sender {
    fn new(ws: WebSocket) -> Self {
        Self { ws }
    }
    pub fn send(&self, msg: Msg) {
        let buffer = flexbuffers::to_vec(&msg).unwrap();
        self.ws.send_with_u8_array(&buffer).unwrap();
    }
}

pub fn connect<F: Fn(Msg, Sender) + 'static>(path: String, route: F) -> Sender {
    let ws =
        WebSocket::new(&format!("ws://localhost:3333/ws{}", path)).expect("to create websocket");
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let sender = Sender::new(ws.clone());
    {
        // onopen
        let onopen_callback = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            log!("websocket open");
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    {
        let onclose_callback = Closure::<dyn FnMut(_)>::new(move |close_event: CloseEvent| {
            log!("websocket closed: {:?}", close_event);
        });
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
    }

    {
        // onmessage
        let onmessage_callback =
            Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| match coldmod_msg(&e) {
                Ok(msg) => route(msg, sender.clone()),
                Err(err) => error!("websocket message error: {:?}", err),
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

    Sender::new(ws)
}

fn coldmod_msg(event: &MessageEvent) -> Result<Msg, anyhow::Error> {
    if let Ok(data) = event.data().dyn_into::<js_sys::ArrayBuffer>() {
        let buffer = js_sys::Uint8Array::new(&data).to_vec();
        return match flexbuffers::from_slice(&buffer) {
            Ok(msg) => Ok(msg),
            Err(err) => Err(anyhow::anyhow!(
                "cant decode to Msg({:?}): {:?}",
                err,
                event.data()
            )),
        };
    }
    Err(anyhow::anyhow!("unexpected message: {:?}", event.data()))
}
