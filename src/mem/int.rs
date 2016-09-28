use {ByteSize};
use std::cmp::min;
use std::iter::repeat;
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
    fn into_vec<'x>(&self, timestamps: &S, dest: &mut Vec<Option<T>>,
        mut num: usize)
    {
        use super::int_deltabuf::Delta::*;
        let diff = min(timestamps.current_age() - self.age, num as u64);
        dest.extend(repeat(None).take(diff as usize));
        if diff >= num as u64 {
            return;
        }
        num -= diff as usize;
        let mut value = self.tip;
        for item in self.buf.deltas() {
            if num == 0 {
                return;
            }
            num -= 1;
            match item {
                Positive(x) => {
                    dest.push(Some(value));
                    value = value - x;
                }
                Negative(x) => {
                    dest.push(Some(value));
                    value = value + x;
                }
                Skip => {
                    dest.push(None);
                }
            }
        }
        dest.push(Some(value));
    }
    fn truncate(&mut self, timestamps: &S, num: usize) -> bool {
        let diff = timestamps.current_age() - self.age;
        if diff < num as u64 {
            self.buf.truncate(num - diff as usize);
            return true;
        } else {
            self.buf.truncate(0);
            return false;
        }
    }
}

#[cfg(test)]
mod test {
    use std::usize;
    use mem::{TimestampsMs, IntSeries, Metric};

    quickcheck! {
        fn check_u64(start: u64, vec: Vec<Option<u64>>) -> bool {
            println!("VEC {}, {:?}", start, vec);
            let mut tsv = 1;
            let mut ts =  TimestampsMs::new(tsv);
            let mut vals = IntSeries::new(&ts, start);
            for val in &vec {
                tsv += 1;
                ts.push(tsv);
                val.map(|x| vals.push(&ts, x));
            }
            let mut out = Vec::with_capacity(vec.len());
            vals.into_vec(&ts, &mut out, usize::MAX);
            println!("OUT {:?} {:?} {}", &vec[..], out,
                Some(start) == out[0] &&
                &vec[..] == &out[1..]);
            out.reverse();
            return
                Some(start) == out[0] &&
                &vec[..] == &out[1..];
        }
    }
}
