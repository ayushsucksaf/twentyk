use std::io::{self, Write};
use crate::database::{get_month_time, get_today_time, get_week_time, connect_to_database};
pub fn print_main_menu() {
    // clear_screen(); // clean first, in case it was called again
    //print the main menu
    // println!("20K");
    println!("•start   •stats    •exit");
    print!("> ");
    io::stdout().flush().unwrap();

}

pub fn print_countdown_screen(){
    clear_screen();
    println!("20K");
    println!("press ctrl+c to stop");
    io::stdout().flush().unwrap();
}
pub fn clear_screen() {
    //clear all the lines and then move the cursor to the top
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn stats_screen(){
    let conn = connect_to_database().unwrap();
    clear_screen();
    println!("20K");
    println!("Here are your stats:");
    // get today, week, and month's time respectively in seconds
    let today_time: u64 = get_today_time(&conn).unwrap() as u64;
    let week_time: u64 = get_week_time(&conn).unwrap() as u64;
    let month_time: u64 = get_month_time(&conn).unwrap() as u64;
    // converting that to hours, minutes, and seconds respectively
    let re_today_time = convert_to_h_m_s(today_time);
    let re_week_time = convert_to_h_m_s(week_time);
    let re_month_time = convert_to_h_m_s(month_time);
    // printing each of them
    println!("Today's time: {} Hours, {} Minutes and {} Seconds", re_today_time.0, re_today_time.1, re_today_time.2);
    println!("Week's time: {} Hours, {} Minutes and {} Seconds", re_week_time.0, re_week_time.1, re_week_time.2);
    println!("Month's time: {} Hours, {} Minutes and {} Seconds", re_month_time.0, re_month_time.1, re_month_time.2);
}

fn convert_to_h_m_s(seconds: u64) -> (u64, u64, u64){
    // this function takes in seconds and converts them into hours, minutes, and seconds respectively then returns em
    let time_elapsed = seconds;
    let hour = time_elapsed/3600;
    let minute = (time_elapsed % 3600) / 60;
    let seconds = time_elapsed - hour * 3600 - minute * 60;
    (hour, minute, seconds)
}