use super::int_deltabuf::DeltaBuf;
use super::Timestamps;


/// Timestamp series with millisecond granularity
pub struct TimestampsMs {
    last: u64,
    age: u64,
    buf: DeltaBuf<u64>,
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
    /// Push next timestamp into timestamp history
    ///
    /// Note, you must push timestamp first, before pushing value into metrics
    /// that depends on this timestamp storage.
    pub fn push(&mut self, timestamp: u64) {
        self.buf.push(self.last, timestamp, 1);
        self.last = timestamp;
        self.age += 1;
    }
}

impl Timestamps for TimestampsMs {
    type Age = u64;
    fn current_age(&self) -> u64 {
        return self.age;
    }
}
