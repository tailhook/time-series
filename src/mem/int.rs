use super::Integer;
use super::int_deltabuf::DeltaBuf;

struct IntSeries<T:Integer> {
    tip: T,
    age: u64,
    buf: DeltaBuf<T>,
}

fn hello() {
}
