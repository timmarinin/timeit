extern crate time;

use std::env;
use std::path::{Path,PathBuf};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use time::Timespec;

fn arg_to_path(arg: &String) -> PathBuf {
    let mut path = PathBuf::from("/tmp");
    path.push("timeit");
    path.push(arg);
    path.set_extension("tmp");
    path
}

fn assure_directory() {
    let path = Path::new("/tmp/timeit");
    match fs::create_dir(path) {
        Err(why) => match why.kind() {
            ErrorKind::AlreadyExists => {},
            _ => println!("Could not create a directory {}! {}", path.display(), why)
        },
        Ok(_) => println!("Created directory!")
  };
}

fn try_read_file(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    Ok(s)
}

fn parse_time(time: String) -> Timespec {
    let split: Vec<i64> = time
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sec: i64 = split[0];
    let nsec: i32 = split[1] as i32;
    Timespec::new(sec, nsec)
}

fn get_current_time() -> Timespec {
    time::get_time()
}

fn get_diff(a: Timespec, b: Timespec) -> i64 {
    match (a - b).num_nanoseconds() {
        Some(nsec) => nsec,
        None => 0 as i64
    }
}

fn print_diff(task: &String, prev_time: String) {
    let prev = parse_time(prev_time);
    let now = get_current_time();
    let diff = get_diff(now, prev);
    println!("{} {}", task, diff);
}

fn drop_file(path: &Path) {
    match fs::remove_file(path) {
        Ok(_) => {},
        Err(why) => println!("Could not delete old timer file at {}: {}", path.display(), why)
    };
}

fn write_current_time(path: &Path) -> io::Result<()> {
    let now = get_current_time();
    let mut file = try!(File::create(path));
    let out = format!("{} {}", now.sec, now.nsec);
    file.write_all(out.as_bytes())
}

fn handle_existing_timer(task: &String, path: &Path, prev_time: String) {
    print_diff(task, prev_time);
    drop_file(path);
}

fn handle_new_timer(task: &String, path: &Path) {
    match write_current_time(&path) {
        Ok(_) => {},
        Err(why) => println!("Could not write timer {} to {}: {}", task, path.display(), why)
    };
}

fn do_the_thing(task: &String, path: &Path) {
    match try_read_file(&path) {
        Ok(prev_time) => handle_existing_timer(task, path, prev_time),
        Err(why) => match why.kind() {
            ErrorKind::NotFound => handle_new_timer(task, path),
            _ => println!("Could not read previous time from {}! {}", path.display(), why)
        }
    }
}

fn main() {
    assure_directory();

    if let Some(task) = env::args().nth(1) {
        let path = arg_to_path(&task);
        do_the_thing(&task, &path);
    } else {
        println!("timeit requires single argument -- name of the timer");
  }
}
