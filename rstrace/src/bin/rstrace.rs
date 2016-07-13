extern crate rstrace;

use rstrace::tracer;
use std::thread;
use std::time::Duration;



fn main() {
    let s = tracer::current_span();
    thread::sleep(Duration::new(3, 0));
    s.borrow_mut().stop();
    thread::sleep(Duration::new(1, 0));
    s.borrow_mut().add_kv_annotation(Box::new(b"foo".to_vec()), Box::new(b"bar".to_vec()));
    println!("{:?}", s.borrow());
}
