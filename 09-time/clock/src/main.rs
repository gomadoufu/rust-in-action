use chrono::DateTime;
use chrono::Local;

struct Clock;

fn main() {
    let now = Local::now();
    println!("{}", now);
}
