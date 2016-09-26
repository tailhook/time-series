use std::time::{SystemTime, UNIX_EPOCH};

use super::int_deltabuf::DeltaBuf;
use super::Timestamps;

use {ByteSize};


/// Timestamp series with millisecond granularity
pub struct TimestampsMs {
    last: u64,
    age: u64,
    buf: DeltaBuf<u64>,
}

fn time_to_ms(time: SystemTime) -> u64 {
    let d = time.duration_since(UNIX_EPOCH).unwrap();
    return d.as_secs()*1000 + (d.subsec_nanos() / 1000_000) as u64;
}

impl TimestampsMs {
    /// New timestamp history starting with a specified timestamp
    pub fn new(timestamp: u64) -> TimestampsMs {
        TimestampsMs {
            age: 0,
            last: timestamp,
            buf: DeltaBuf::new(),
        }
    }
    /// Same as new but current current system time
    pub fn new_now() -> TimestampsMs {
        TimestampsMs::new(time_to_ms(SystemTime::now()))
    }
    /// Push next timestamp into timestamp history
    ///
    /// Note, you must push timestamp first, before pushing value into metrics
    /// that depends on this timestamp storage.
    pub fn push(&mut self, timestamp: u64) {
        self.buf.push(self.last, timestamp, 1);
        self.last = timestamp;
        self.age += 1;
    }
    /// Same as push but uses current system time
    pub fn push_now(&mut self) {
        self.push(time_to_ms(SystemTime::now()))
    }
}

impl Timestamps for TimestampsMs {
    fn current_age(&self) -> u64 {
        return self.age;
    }
}

impl ByteSize for TimestampsMs {
    fn size(&self) -> usize {
        ::std::mem::size_of_val(self) -
            ::std::mem::size_of_val(&self.buf) + self.buf.size()
    }
}
