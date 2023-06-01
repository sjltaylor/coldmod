use crate::events::AppEvent;

#[derive(Clone)]
pub struct Dispatch {
    ch: (
        crossfire::mpmc::TxUnbounded<AppEvent>,
        crossfire::mpmc::RxUnbounded<AppEvent>,
    ),
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            ch: crossfire::mpmc::unbounded_future::<AppEvent>(),
        }
    }
    pub fn send(&self, event: AppEvent) -> Result<(), anyhow::Error> {
        self.ch
            .0
            .send(event)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        Ok(())
    }
    pub async fn receive(&self) -> Result<AppEvent, anyhow::Error> {
        self.ch
            .1
            .recv()
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))
    }
}
