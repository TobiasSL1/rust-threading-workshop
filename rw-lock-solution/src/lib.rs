use std::{sync::RwLock, thread};

#[test]
fn it_works() {
    let rw = &RwLock::new(String::new());

    thread::scope(|s| {
        (0u8..5u8).into_iter().for_each(|number| {
            s.spawn(move || {
                let mut var = rw.write().unwrap();
                (*var).push(char::from_u32((b'a' + number) as u32).unwrap_or(' '));
            });
        });
    });

    assert_eq!(rw.read().unwrap().len(), 5);

    let result = rw.read().unwrap();
    assert_ne!(result.find('a'), None);
    _= dbg!(result);
    //assert_ne!(rw.read().unwrap().find('a'), None)
}
