use std::sync::mpsc;
use std::thread;

fn main() {
   let ( tx, rx) = mpsc::channel::<String>();

   thread::spawn({
      let tx1 = tx.clone();
      move || tx1.send(String::from("from thread 1"))
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
