use chrono::prelude::Local;

fn main() {

    let time = Local::now().format("%-I:%M");

    println!("{}", time);
}