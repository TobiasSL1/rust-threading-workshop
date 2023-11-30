use std::sync::mpsc;
use std::thread;

fn main() {
   let ( tx, rx) = mpsc::channel::<String>();

   let tx1 = tx.clone();
   thread::spawn(move || {
      tx1.send(String::from("from thread 1"))
   });

   thread::spawn(move || {
      tx.send(String::from("from thread 2"))
   });

   for _ in 0..2
   {
      let val = rx.recv().unwrap();
      println!("{}", val)
   }
}
