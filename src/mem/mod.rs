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

/// Timetsamps
pub trait Timestamps {
    /// Age of the current (most recent) timestamp
    ///
    /// This function should ouly be used in Metric for optimisations
    fn current_age(&self) -> u64;
}

/// Generic interface for storing history of a single metric
pub trait Metric<S: Timestamps>: Sized {
    /// Single value type
    type Value;
    /// Pash a new value
    fn push(&mut self, timestamps: &S, value: Self::Value)
        -> Result<(), PushError>;
    /// Pushes last values from this metric into a vector
    ///
    /// Values are pushed with most recent value first
    ///
    /// This is implemented in the spirit of `io::Read::read` rather than
    /// as an iterator for two reasons:
    ///
    /// 1. We want maximum performance so using preallocated vector is good
    /// 2. We want "object safe" trait so we can't use generics here
    /// 3. If it were iterator every implementation must return it's own
    ///    iterator type, which will not work without some kind of boxing
    ///
    /// Why use use a separate `max` parameter:
    ///
    /// * `max` doesn't equal to capacity because that would mean reallocating
    ///   vector when not needed (i.e. either shriking it before passing a
    ///   function, or growing when there are no actual data)
    /// * we don't use slice and return value because that would mean
    ///   unnecessarily zeroing vector or security issues
    fn into_vec(&self, timestamps: &S, dest: &mut Vec<Option<Self::Value>>,
                max: usize);

    /// Truncate history up to `num` values at max
    ///
    /// Returns `true` if there are some data left in the storage
    fn truncate(&mut self, timestamps: &S, num: usize) -> bool;
}


quick_error! {
    /// Error when pusing value into a metric buffer
    #[derive(Debug)]
    pub enum PushError {
        DuplicateValue {
            description("received a value for the same timestamp twice")
        }
    }
}
