use std::path::PathBuf;
use rusqlite::{params, Connection, Result};
use chrono::{NaiveDate, Local, Days};
use directories::ProjectDirs;

fn get_db_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "20k").expect("Couldn't get project dirs");
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir).unwrap(); // ensure it exists
    data_dir.join("time_storage.db")
}

pub fn get_todays_date() -> NaiveDate {
    // function returns today's date in NaiveDate
    let naive_dt = Local::now().date_naive();
    naive_dt
}
pub fn connect_to_database()-> Result<Connection, String> {
    // function creates a connection to the database "time_storage.db"
    let conn = match Connection::open(get_db_path()) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    // sql query to create the table or check if it already exists
    let sql_query = "CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
    date TEXT NOT NULL,
    time_in_seconds INTEGER NOT NULL
    )";

    // executes the given sql query using rusqlite
    match conn.execute(sql_query, []) {
        Ok(_) => Ok(conn),
        Err(e) => Err(e.to_string()),
    }

}
pub fn clock_in_time(conn: &Connection, time_in_seconds: i64 )-> Result<(), String> {
    // function to upload the time worked into the database
    let date_today = get_todays_date().to_string();

    // simple insert sql query
    let sql_query = "INSERT INTO sessions (date, time_in_seconds)
    VALUES (?, ?)";

    match conn.execute(sql_query, params![date_today, time_in_seconds]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())

    }
}

pub fn get_today_time(conn: &Connection) -> Result<i64, String> {
    // function to get the hours clocked in that day
    let date_today_string = get_todays_date().to_string();

    // simple sql query to get the cumulative seconds of that day
    let sql_query = "SELECT COALESCE(SUM(time_in_seconds), 0) as today_time FROM sessions WHERE date = ?";
    match conn.query_row(sql_query, params![date_today_string], |row| row.get(0)) {
        Ok(total) => Ok(total),
        Err(e) => Err(e.to_string())
    }
}

pub fn get_week_time(conn: &Connection) -> Result<i64, String> {
    // function to get the hours clocked in that week (from 7 days before that day)
    let date_now = get_todays_date();
    let the_subtractor = Days::new(7); // creating a new date object so we can use sub_days to subtract 7 days from that day
    let week_ago_date = date_now.checked_sub_days(the_subtractor).unwrap();
    let week_ago_date_string = week_ago_date.to_string(); //  converted to string for sql operations
    let sql_query = "SELECT COALESCE(SUM(time_in_seconds), 0) as week_time FROM sessions WHERE date >= ?";
    match conn.query_row(sql_query, params![week_ago_date_string], |row| row.get(0)) {
        Ok(total) => Ok(total),
        Err(e) => Err(e.to_string())
    }
}

pub fn get_month_time(conn: &Connection) -> Result<i64, String> {
    // function to get the hours clocked in that month (from 30 days before that day)
    let date_now = get_todays_date();
    let the_subtractor = Days::new(30); // creating a new date object so we can use sub_days to subtract 30 days from that day
    let month_ago_date = date_now.checked_sub_days(the_subtractor).unwrap();
    let month_ago_date_string = month_ago_date.to_string();
    let sql_query = "SELECT COALESCE(SUM(time_in_seconds), 0) as week_time FROM sessions WHERE date >= ?";
    match conn.query_row(sql_query, params![month_ago_date_string], |row| row.get(0)) {
        Ok(total) => Ok(total),
        Err(e) => Err(e.to_string())
    }
}


