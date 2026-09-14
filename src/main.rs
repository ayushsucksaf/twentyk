mod ui;
mod tracker;
mod database;

use ui::{print_main_menu, stats_screen};
use::std::io::{self};
use tracker::start_tracking;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_for_handler = running.clone();

    ctrlc::set_handler(move || {
        running_for_handler.store(false, Ordering::SeqCst);
    })
        .expect("error in handler");

    println!("20K");
    print_main_menu();
    loop {
        // print_main_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        match command {
            "start" => {
                running.store(true, Ordering::SeqCst); // reset for a new session
                start_tracking(running.clone());
                print_main_menu();
            }
            "stats" => {
                stats_screen();
                print_main_menu();
            }
            "exit" => {
                break;
            }
            _ => {println!("Unknown command {}", command);
            break;}
        }
    }
}

