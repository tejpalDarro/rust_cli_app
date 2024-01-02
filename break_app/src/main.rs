use std::time::Duration;
use std::thread::sleep;
fn main() {
    for i  in 1..11 {
        sleep(Duration::from_millis(1000));
        println!("{}", i);
    }
}
