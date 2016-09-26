extern crate time_series;

use std::io::{self, stdin, stderr, Write};
use time_series::mem::{TimestampsMs, IntSeries, Metric};
use time_series::ByteSize;


fn read_value() -> Result<Option<u64>, io::Error> {
    loop {
        let mut buf = String::with_capacity(100);
        if try!(stdin().read_line(&mut buf)) == 0 {
            return Ok(None);
        }
        match buf.trim().parse() {
            Ok(x) => return Ok(Some(x)),
            Err(e) => {
                writeln!(&mut stderr(), "Error parsing int: {}", e).ok();
            }
        }
    }
}


fn main() {
    println!("Type numbers, use Ctrl+D when done");
    let val =
        match read_value() {
            Ok(Some(x)) => x,
            Ok(None) => return,
            Err(e) => {
                writeln!(&mut stderr(), "Error reading: {}", e).ok();
                return;
            }
        };

    let mut timestamps = TimestampsMs::new_now();
    let mut values = IntSeries::new(&timestamps, val);
    let mut num = 1;

    loop {
        match read_value() {
            Ok(Some(x)) => {
                timestamps.push_now();
                values.push(&timestamps, x).expect("Push value");
                num += 1;
            }
            Ok(None) => break,
            Err(e) => {
                writeln!(&mut stderr(), "Error reading: {}", e).ok();
                break;
            }
        }
    }
    println!("Stored {} values in {} + {} = {} bytes",
        num,
        timestamps.size(), values.size(),
        timestamps.size() + values.size());
}
