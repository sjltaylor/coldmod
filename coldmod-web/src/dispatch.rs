use crate::events::AppEvent;
use coldmod_msg::web::Msg;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Dispatch {
    app_event_cbs: Rc<RefCell<Vec<Box<dyn FnMut(AppEvent) + 'static>>>>,
    _app_event_cbs_q: Rc<RefCell<Vec<Box<dyn FnMut(AppEvent) + 'static>>>>,
    _calling_back: Rc<RefCell<bool>>,
}

impl Dispatch {
    pub fn new() -> Self {
        Self {
            app_event_cbs: Rc::new(RefCell::new(Vec::new())),
            _app_event_cbs_q: Rc::new(RefCell::new(Vec::new())),
            _calling_back: Rc::new(RefCell::new(false)),
        }
    }

    pub fn on_app_event(&self, cb: impl FnMut(AppEvent) + 'static) {
        if *self._calling_back.borrow() {
            self._app_event_cbs_q.borrow_mut().push(Box::new(cb));
        } else {
            self.app_event_cbs.borrow_mut().push(Box::new(cb));
        }
    }

    pub fn emit(&self, event: AppEvent) {
        *self._calling_back.borrow_mut() = true;

        self.app_event_cbs
            .borrow_mut()
            .iter_mut()
            .map(|cb| cb(event.clone()))
            .count();

        *self._calling_back.borrow_mut() = false;

        let mut q = self._app_event_cbs_q.borrow_mut();

        if !q.is_empty() {
            self.app_event_cbs.borrow_mut().append(&mut q);
        }
    }
}
