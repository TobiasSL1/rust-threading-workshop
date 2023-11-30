use std::{thread, time};

fn main() {
    let mut str: String =  std::env::var("USER").unwrap_or("unknown".to_owned());

    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(time::Duration::from_millis(1));
            str += "."
        });
    });
    println!("Hello, world {} !", str);
}
