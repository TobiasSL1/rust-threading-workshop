
use std::sync::{Arc, Weak};

struct A{
}

impl Drop for A {
    fn drop(&mut self) {
        println!("A dropped");
    }
}

#[test]
fn main() {
    let r = Arc::new(A{});
    let w = Arc::downgrade(&r);
    println!("will upgrade");
    assert!(w.upgrade().is_some());
    println!("will drop arc");
    drop(r);
    println!("arc dropped");
    assert!(w.upgrade().is_none());
}
