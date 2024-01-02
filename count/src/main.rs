use std::env;
use std::{thread, time};
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    // let one_sec = time::Duration::from_secs(1);
    // let args: Vec<String> = env::args().collect();
    // let n: i32 = args[1].parse().unwrap();
    // 
    // for i in (0..n).rev() {
    //     println!("{}", i);
    //     thread::sleep(one_sec);
    // }
    let one_sec = time::Duration::from_secs(1);
    for i in (1..=10).rev() {
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("Countdown")
            .text1(&i.to_string())
            .duration(Duration::Short)
            .sound(Some(Sound::IM))
            .show()
            .expect("unable to toast");
        thread::sleep(one_sec);
    }
}
