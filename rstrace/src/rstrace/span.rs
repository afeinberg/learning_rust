extern crate rand;

use std::vec::Vec;
use std::collections::BTreeMap;
use std::time::{UNIX_EPOCH, SystemTime, Instant, Duration};
use std::fmt;

use rand::Rng;
use rand::os::OsRng;

pub type BytesMapIterator<'a> = Iterator<Item = (&'a Box<Vec<u8>>, &'a Box<Vec<u8>>)> + 'a;

pub trait ToMillis {
    fn to_millis(&self) -> u64;
}

impl ToMillis for Duration {
    fn to_millis(&self) -> u64 {
        self.as_secs() * 1000 + self.subsec_nanos() as u64 / 1000000
    }
}

pub trait Span {
    fn stop(&mut self);
    fn acc_millis(&self) -> u64;
    fn is_running(&self) -> bool;
    fn child(&mut self, description: &str) -> Self where Self: Sized;
    fn add_kv_annotation(&mut self, key: Box<Vec<u8>>, value: Box<Vec<u8>>);
    fn kv_annotations<'a>(&'a self) -> Box<BytesMapIterator<'a>>;
    fn started_at_millis(&self) -> u64;
    fn trace_id(&self) -> u64;
    fn span_id(&self) -> u64;
    fn parent_span_id(&self) -> u64;
    fn description(&self) -> String;
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kv_annotations = self.kv_annotations().fold(String::new(), |s, (k, v)| {
            format!("{}{}{}={}",
                    s,
                    if s == "" {
                        "["
                    } else {
                        ","
                    },
                    String::from_utf8(*k.clone()).unwrap(),
                    String::from_utf8(*v.clone()).unwrap())
        });
        write!(f,
               "Span[id: {}, description: {}, parent_span_id: {}, trace_id: {}, acc_millis: {}, \
                started_at: {}, kv_annotations: {}]]",
               self.span_id(),
               self.description(),
               self.parent_span_id(),
               self.trace_id(),
               self.acc_millis(),
               self.started_at_millis(),
               kv_annotations)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct MilliSpan {
    description: String,
    is_running: bool,
    started_at: Instant,
    annotations: BTreeMap<Box<Vec<u8>>, Box<Vec<u8>>>,
    trace_id: u64,
    acc_millis: u64,
    started_at_millis: u64,
    parent_span_id: u64,
    span_id: u64,
    rng: OsRng,
}


impl MilliSpan {
    pub fn new(description: &str, trace_id: u64, parent_span_id: u64, span_id: u64) -> Self {
        MilliSpan {
            description: String::from(description),
            is_running: true,
            started_at: Instant::now(),
            annotations: BTreeMap::new(),
            trace_id: trace_id,
            acc_millis: 0,
            started_at_millis: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .to_millis(),
            parent_span_id: parent_span_id,
            span_id: span_id,
            rng: OsRng::new().unwrap(),
        }
    }
}

pub const ROOT_SPAN_ID: u64 = 0x74acb;

impl Span for MilliSpan {
    fn stop(&mut self) {
        if self.is_running {
            self.acc_millis = self.acc_millis();
            self.is_running = false;
        }
    }
    fn acc_millis(&self) -> u64 {
        if self.is_running {
            self.started_at.elapsed().to_millis()
        } else {
            self.acc_millis
        }
    }
    fn is_running(&self) -> bool {
        self.is_running
    }
    fn add_kv_annotation(&mut self, key: Box<Vec<u8>>, value: Box<Vec<u8>>) {
        self.annotations.insert(key, value);
    }
    fn kv_annotations<'a>(&'a self) -> Box<BytesMapIterator<'a>> {
        Box::new(self.annotations.iter())
    }
    fn started_at_millis(&self) -> u64 {
        self.started_at_millis
    }
    fn span_id(&self) -> u64 {
        self.span_id
    }
    fn trace_id(&self) -> u64 {
        self.trace_id
    }
    fn description(&self) -> String {
        self.description.clone()
    }
    fn parent_span_id(&self) -> u64 {
        self.parent_span_id
    }
    fn child(&mut self, description: &str) -> Self {
        MilliSpan::new(description,
                       self.trace_id,
                       self.span_id,
                       self.rng.next_u64())
    }
}
