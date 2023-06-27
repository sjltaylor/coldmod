use std::{cell::RefCell, rc::Rc, time::Duration};

use coldmod_msg::web::Msg;

use crate::events::AppEvent;

use leptos::*;

#[derive(Clone)]
pub struct Dispatch {
    app_event_cbs: Rc<RefCell<Vec<Box<dyn FnMut(AppEvent) + 'static>>>>,
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            app_event_cbs: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn on_app_event(&self, cb: impl FnMut(AppEvent) + 'static) {
        let later = self.clone();
        set_timeout(
            move || {
                later.app_event_cbs.borrow_mut().push(Box::new(cb));
            },
            Duration::from_nanos(0),
        );
    }

    pub fn emit(&self, event: AppEvent) {
        match event {
            AppEvent::ColdmodMsg(Msg::HeatMapChanged(ref heatmap_delta)) => {
                for delta in heatmap_delta.deltas.iter() {
                    let (k, d) = (delta.0.clone(), *delta.1);
                    self.emit(AppEvent::SourceElementTraceCountChanged((k, d)));
                }
            }
            _ => {}
        };
        self.app_event_cbs
            .borrow_mut()
            .iter_mut()
            .map(|cb| cb(event.clone()))
            .count();
    }
}
