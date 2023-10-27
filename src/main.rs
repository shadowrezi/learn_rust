use std::{
    io::{
        stdin,
        BufReader
    },
    fs::File,
    time::Duration,
    thread::sleep
};
use rodio::{
    Source,
    Decoder
};


fn main() {
    let mut x = "".to_string();

    loop {
        match stdin().read_line(&mut x) {
            Ok(_) => {},
            Err(_) => {}
        }
        let time = x.trim().parse::<i32>().unwrap();
    
        timer(time);
    }
}

fn timer(time: i32) {
    for i in 0..=time {
        println!("{} secs elapsed!", i);

        sleep(Duration::from_secs(1));
    }

    //* Play audio ringtone
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = File::open("iphone_14.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    
    let _ = stream_handle.play_raw(source.convert_samples());
}