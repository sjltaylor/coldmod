use leptos::*;

#[derive(Clone, Debug)]
pub enum AppView {
    Source,
    Trace,
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    NavigateTo(AppView),
}

pub type EventsChannel = (
    async_channel::Sender<AppEvent>,
    async_channel::Receiver<AppEvent>,
);
