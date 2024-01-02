use chrono::{Datelike , NaiveDate,DateTime, Local, FixedOffset, Utc};
use colored::Colorize;
use now::{TimeZoneNow, DateTimeNow};


fn main() {
    let local: DateTime<Local> = Local::now();
    println!("week: {}", local.week_of_year());
    let time = Utc::now();
    println!("time {}", time.beginning_of_week());
    let _vec = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let _len = 42;
    let _day = local.day();
    let mut _skip_day = 5;

    let last_day = NaiveDate::from_ymd(local.year(), local.month()+1 , 1)
        .pred().day();

    println!();
    println!("    {}",local.format("%A, %B %e, %Y"));
    println!();
    
    let _weekday = local.weekday().to_string();
    println!("day {}", _day);
    println!("weekday: {}", _weekday);
    for ele in 0.._vec.len() {
        println!("week: {}", _vec[ele]);
        if _weekday.eq(_vec[ele]){
            println!("find it");
            _skip_day = ele;
            break;
        }
    }
    println!();
    println!("skip day: {}", _skip_day);

    for weeks in _vec {
        print!(" {0: <4}", weeks);
    }
    println!();
    let mut _count = 1;
    for _x in 1..=42 {
        if _skip_day >= _x {
            print!(" {0: <4}", " ");
        } else {
            if _day == _count {
                print!(" {0: <4}", _count.to_string().green());
                _count+=1;
            } else {
                if _count <= last_day {
                    print!(" {0: <4}", _count);
                } else {
                    print!(" {0: <4}", " ");
                }
                _count+=1;
            }
        }
        if _x % 7 == 0 {
            println!();
        }
    }
}

