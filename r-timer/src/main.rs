extern crate chrono;

use std::{time::{SystemTime}};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use std::env;
use notify_rust::{Notification,Timeout};
use cursive::views::TextView;
use cursive::views::Dialog;

fn calculate_seconds_to_alarm(start: DateTime<Utc>, end: DateTime<Utc>) -> Option<u64> {
    let seconds = (end - start).num_seconds();
    if seconds >= 0 {
        Some(seconds as u64)
    } else {
        if seconds < 0 {
            println!("Error: Unable to set alarm for time that is in the past... Unless you have a TARDIS?");
        }
        None
    }
}

fn generate_datetime(time: &str) -> Result<DateTime<Utc>, String> {
    let (h, m, s) = parse_hms(time)?;

    let now = Utc::now();

    Utc.with_ymd_and_hms(
        now.year(),
        now.month(),
        now.day(),
        h,
        m,
        s,
    )
    .single()
    .ok_or("Invalid date/time".into())
}



fn parse_hms(time: &str) -> Result<(u32, u32, u32), String> {
    let parts: Vec<_> = time.split(':').collect();

    let [h, m, s] = parts.as_slice() else {
        return Err("Expected time in HH:MM:SS format".into());
    };

    let h: u32 = h.parse().map_err(|_| "Invalid hour")?;
    let m: u32 = m.parse().map_err(|_| "Invalid minute")?;
    let s: u32 = s.parse().map_err(|_| "Invalid second")?;

    Ok((h, m, s))
}


fn parse_time_arg(args: Vec<String>) -> Result<DateTime<Utc>, String> {
    let time = args.get(1).ok_or("Expected HH:MM:SS argument")?;
    generate_datetime(time)
}


fn main() {
    let mut siv = cursive::default();
    let args: Vec<String> = env::args().collect();
    let start: DateTime<Utc> = Utc::now();

    let parsed_time: DateTime<Utc> = match parse_time_arg(args) {
        Ok(dt) => dt,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    
    let seconds_to_wait = match calculate_seconds_to_alarm(start, parsed_time) {
        Some(seconds) => seconds,
        None => {
            eprintln!("Error: could not convert | {start} | or | {parsed_time} | into seconds.");
            std::process::exit(1);
        }
    };


    let busy_loop = true;

    while busy_loop{
        
        match SystemTime::now().duration_since(start.into()) {
            Ok(n) => {
                if n.as_secs() > seconds_to_wait{
                    break;
                }
                else {
                    println!("{}s remaining",seconds_to_wait-n.as_secs());
                    print!("{}[2J", 27 as char);
                    
                }
            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }

    }

    
    let body_string = format!("Timer finished at {}:{}:{}",parsed_time.hour(),parsed_time.minute(),parsed_time.second());


    Notification::new()
    .summary("Timer finished!")
    .body(&body_string)
    .icon("time")
    .timeout(Timeout::Milliseconds(6000)) //milliseconds
    .show().unwrap();
    siv.add_layer(Dialog::around(TextView::new("Timer finished!"))
                         .title("Timer")
                         .button("Quit", |s| s.quit()));

    siv.run();

   
}