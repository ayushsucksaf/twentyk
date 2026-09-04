use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Instant, Duration};
use std::io::{self, Write};
use crate::ui::{clear_screen, print_countdown_screen};
use crate::database::{connect_to_database, clock_in_time};
use std::thread::sleep;

pub fn start_tracking(running: Arc<AtomicBool>) {
    // start point
    let now = Instant::now();

    // sleep function
    sleep_and_run_countdown(running);

    // this prints at the end after user asks to stop
    clear_screen();
    let time_elapsed = now.elapsed().as_secs();
    let hour = time_elapsed/3600;
    let minute = (time_elapsed % 3600) / 60;
    let seconds = time_elapsed - hour * 3600 - minute * 60;
    println!("Time clocked: {} Hours, {} Minutes and {} Seconds", hour, minute, seconds);

}

pub fn sleep_and_run_countdown(running: Arc<AtomicBool>) {
    print_countdown_screen();
    let start = Instant::now();
    let mut saved:u64 = 0;
    //using a saved variable to save the time for every minute

    while running.load(Ordering::SeqCst) {
        // 1 second for the cli
        sleep(Duration::from_secs(1));
        let elapsed = start.elapsed().as_secs();
        let hour = elapsed/3600;
        let minute = (elapsed % 3600) / 60;
        let seconds = elapsed - hour * 3600 - minute * 60;
        print!("\rElapsed: {}hr : {}min : {}s", hour, minute, seconds);
        // checking when 60secs pass
        if elapsed - saved == 60{
            save_to_db(60);
            saved = elapsed;
        }
        io::stdout().flush().unwrap();
    }
}

fn save_to_db(elapsed: u64){
    // the saving function
    let conn = connect_to_database().unwrap();
    clock_in_time(&conn, elapsed as i64).unwrap();
}