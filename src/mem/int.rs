use {ByteSize};
use std::marker::PhantomData;
use super::{Integer, Timestamps, Metric, PushError};
use super::int_deltabuf::DeltaBuf;


pub struct IntSeries<S:Timestamps, T:Integer> {
    tip: T,
    age: u64,
    buf: DeltaBuf<T>,
    phantom: PhantomData<S>,
}

impl<S:Timestamps, T:Integer> IntSeries<S, T> {
    pub fn new(timestamps: &S, value: T) -> IntSeries<S, T> {
        IntSeries {
            tip: value,
            age: timestamps.current_age(),
            buf: DeltaBuf::new(),
            phantom: PhantomData,
        }
    }
}

impl<S:Timestamps, T:Integer> ByteSize for IntSeries<S, T> {
    fn size(&self) -> usize {
        ::std::mem::size_of_val(self) -
            ::std::mem::size_of_val(&self.buf) + self.buf.size()
    }
}

impl<S:Timestamps, T:Integer> Metric<S> for IntSeries<S, T> {
    type Value = T;
    fn push(&mut self, timestamps: &S, value: Self::Value)
        -> Result<(), PushError>
    {
        let diff = timestamps.current_age() - self.age;
        if diff == 0 {
            return Err(PushError::DuplicateValue);
        }
        self.buf.push(self.tip, value, diff);
        self.tip = value;
        self.age = timestamps.current_age();
        Ok(())
    }
}
