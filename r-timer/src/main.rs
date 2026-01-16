extern crate chrono;
use core::panic;
use std::{time::{SystemTime}};
use chrono::{TimeZone, Utc,DateTime};
use notify_rust::Notification;

// might need to make references to allow borrowing
fn calculate_seconds_to_alarm(start: DateTime<Utc>, end: DateTime<Utc>) -> u64 {
    let elapsed: i64  = (end-start).num_seconds();
    let elapsed: u64 = match u64::try_from(elapsed){
        Ok(num) => num,
        Err(e) => panic!("Error! Could not convert {elapsed} into unsigned variant.\n\n\tERR: {e:?}")
    };
    return elapsed;
}


fn main() {

    let start: DateTime<Utc> = Utc::now();
    let end: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 16, 21, 0, 0).unwrap();
    let seconds : u64 = calculate_seconds_to_alarm(start,end);

    let busy_loop = true;

    while busy_loop{
        
        match SystemTime::now().duration_since(start.into()) {
            Ok(n) => {
                if n.as_secs() > seconds{
                    break;
                }
            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    
    }

//     Notification::new().show();
}