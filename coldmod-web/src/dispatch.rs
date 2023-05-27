use crate::events::AppEvent;

#[derive(Clone, Debug)]
pub struct Dispatch {
    pub channel: (
        async_channel::Sender<AppEvent>,
        async_channel::Receiver<AppEvent>,
    ),
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            channel: async_channel::bounded::<AppEvent>(65536),
        }
    }
}
