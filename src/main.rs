use std::{
    thread,
    time::{self, Duration},
};

const SLEEP_FOR: Duration = time::Duration::from_millis(100);
const GREEN_SQUARE: &str = "\x1b[48;2;0;255;0m  \x1b[0m";
const WIDTH: usize = 40;
const HEIGHT: usize = 20;

fn main() {
    println!("{esc}c", esc = 27 as char);
    loop {
        thread::sleep(SLEEP_FOR);
        println!("{esc}c", esc = 27 as char);
        println!("{}", GREEN_SQUARE);
    }
}
