use web_sys::CloseEvent;

#[derive(Clone, Debug)]
pub enum AppEvent {
    ColdmodMsg(coldmod_msg::web::Msg),
    WebSocketClientEvent(WebSocketEventType),
}

#[derive(Clone, Debug)]
pub enum WebSocketEventType {
    Close(CloseEvent),
    Open,
}
