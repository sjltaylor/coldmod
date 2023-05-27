use web_sys::CloseEvent;

#[derive(Clone)]
pub enum AppEvent {
    ColdmodMsg(coldmod_msg::web::Event),
    WebSocketClientEvent(WebSocketEventType),
}

#[derive(Clone)]
pub enum WebSocketEventType {
    Close(CloseEvent),
}
