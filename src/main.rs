use chrono::prelude::Local;

fn main() {

    let time = Local::now().format("%-I:%M");

    println!("{}", time);
    println!("{}", test().unwrap());

}


fn test() -> Option<f64> {
    return Some(3.141235123123211);
}
