//extern crate thread_local;
extern crate rand;

use rand::Rng;
use std::borrow::BorrowMut;
use rand::os::OsRng;

use std::rc::Rc;
use std::cell::RefCell;

use span;
use span::{MilliSpan, Span};


pub struct Tracer {
    rng: OsRng,
    current_span: Option<Rc<RefCell<Span>>>,
}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {
            rng: OsRng::new().unwrap(),
            current_span: None,
        }
    }
    fn tracer() -> RefCell<Tracer> {
        RefCell::new(Tracer::new())
    }
    pub fn create_span(&mut self, description: &str) -> Rc<RefCell<Span>> {
        {
            let mut cache = &mut self.current_span;
            if cache.is_some() {
                return cache.as_ref().unwrap().clone();
            }
            let r_1 = self.rng.next_u64();
            let r_2 = self.rng.next_u64();
            *cache = Some(Rc::new(RefCell::new(MilliSpan::new(description,
                                                              r_1,
                                                              span::ROOT_SPAN_ID,
                                                              r_2))));
        }
        self.create_span(description)
    }
}

thread_local!(pub static TRACER: RefCell<Tracer> = Tracer::tracer());

pub fn current_span() -> Rc<RefCell<Span>> {
    TRACER.with(|tr| {
        let mut t = tr.borrow_mut();
        t.create_span("foo")
    })
}
