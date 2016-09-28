extern crate time_series;

use std::usize;
use std::io::{stdin, stderr, Write};
use time_series::mem::{TimestampsMs, IntSeries, Metric};
use time_series::ByteSize;


fn main() {
    let mut buf = String::with_capacity(100);
    println!("Type numbers, use Ctrl+D when done");
    let val: i64;
    loop {
        match stdin().read_line(&mut buf) {
            Ok(0) => return,
            Ok(_) => {
                match buf.trim().parse() {
                    Ok(v) => {
                        val = v;
                        break;
                    }
                    Err(e) => {
                        writeln!(&mut stderr(), "Error parsing int: {}", e)
                            .ok();
                        continue;
                    }
                }
            }
            Err(e) => {
                writeln!(&mut stderr(), "Error reading: {}", e).ok();
                return;
            }
        }
    }

    let mut timestamps = TimestampsMs::new_now();
    let mut values = IntSeries::new(&timestamps, val);
    let mut num = 1;

    loop {
        buf.truncate(0);
        match stdin().read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                num += 1;
                timestamps.push_now();
                match buf.trim().parse() {
                    Ok(v) => {
                        values.push(&timestamps, v).unwrap();
                    }
                    Err(e) => {
                        writeln!(&mut stderr(), "Error parsing int: {}", e)
                            .ok();
                    }
                }
            }
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
    let mut v = Vec::new();
    values.into_vec(&timestamps, &mut v, usize::MAX);
    assert_eq!(num, v.len());
    println!("Contents {:?}", v);
}
