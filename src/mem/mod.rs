//! In-memory data structures
//!
//! All in-memory data structures rely on `Timestamps` trait to handle
//! timestamp bookkeeping. Basic idea is that you scan your metrics at
//! a regular intervals and at each timestamp you have loads of metrics, so
//! it's ineffiecient to store a timestamp per metric.
//!
//! So `Timestamps` is a holder for timestamps. Basically it's an array of
//! timestamps, and "age" value for each. The `age` value is an
//! ever-incrementing counter. With first timestamp you captures having an
//! age of `1`.
//!
//! When metric is updated it stores current age of the `Timestamps` array.
//! When you read metrics it can sync timestamps with stored metrics by
//! calculating the difference between current age and last age stored for
//! specific metric. This way if you don't update metric for some time it
//! still doesn't get out of sync with timestamps. When you update metric
//! after some inactivity period it is filled with "undefined" values for
//! all timestamps in-between.
//!
//! **Note**: this pattern works well for tools like `cantal` or `self-meter`
//! crate where you receive lots of metrics with the same timestamp. Or when
//! you quantize metrics by fixed interval. But not when you receive many
//! metrics with arbitrary timestamp and need to keep store precise timestamps.
//! But you may store a `TimestampsMs` for each metric in the latter case.
//! (or you may contribute better implementation)

use std::ops::{Shl, Shr, BitOr, BitAnd};

use num_integer;
use num_traits::{FromPrimitive, ToPrimitive};

mod int_deltabuf;
mod timestamps;
mod int;

pub use self::timestamps::TimestampsMs;
pub use self::int::IntSeries;

pub trait Integer: num_integer::Integer + Copy +
    Shl<u32, Output=Self> + Shr<u32, Output=Self> +
    BitOr<Self, Output=Self> + BitAnd<Self, Output=Self> +
    FromPrimitive + ToPrimitive
{}

pub trait Timestamps {
    fn current_age(&self) -> u64;
}

pub trait Metric<S: Timestamps> {
    type Value;
    fn push(&mut self, timestamps: &S, value: Self::Value)
        -> Result<(), PushError>;
}


quick_error! {
    #[derive(Debug)]
    pub enum PushError {
        DuplicateValue {
            description("received a value for the same timestamp twice")
        }
    }
}
