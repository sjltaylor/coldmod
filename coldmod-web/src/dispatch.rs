use std::{cell::RefCell, rc::Rc};

use crate::events::AppEvent;

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
        self.app_event_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn emit(&self, event: AppEvent) -> usize {
        self.app_event_cbs
            .borrow_mut()
            .iter_mut()
            .map(|cb| cb(event.clone()))
            .count()
    }
}
