use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {

        {
            let (mv, cv) = &*pair2;
            thread::sleep(Duration::from_millis(10));

            let mut started = mv.lock().unwrap();
            println!("Started");
            *started = true;
            cv.notify_one();
        }
    });

    let (mu, cv) = &*pair;

    let mut started = mu.lock().unwrap();

    loop {
        let result = cv.wait_timeout(started, Duration::from_millis(1)).unwrap();

        started = result.0;
        if *started {
            break;
        }
        println!("waiting.");
    }
    /* *
    while !*started {
        println!("waiting.");
        started = cv.wait(started).unwrap();
    }*/
    println!("finished waiting.")
}
