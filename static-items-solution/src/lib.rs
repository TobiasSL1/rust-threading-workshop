use std::{sync::OnceLock, thread};

#[cfg(test)]


static mut VALUE: i32 = 0;
#[test]
fn read_static() {
    assert_eq!(unsafe {VALUE}, 0);
}

#[test]
fn oncecell_should_initialize_cell_once()  //This sample demonstates one-time initialization of a local variable
{
    let cell = std::cell::OnceCell::new();  // OnceCell cannot be static
    let result = cell.get_or_init(|| "Aufwendig...".to_owned());

    let result2 = cell.get_or_init(|| "Aufwendig...!!!".to_owned());

    assert_eq!(result, result2);
}

#[test]
fn once_lock_should_initialize_once()  //This sample demonstates one-time initialization of a local variable
{
    static CELL : OnceLock<String> = OnceLock::new();

    let init_func = || "Aufwendig...".to_owned();

    let threads =  [
         thread::spawn(move || CELL.get_or_init(init_func)),
         thread::spawn(move || CELL.get_or_init(init_func)),
    ];

    threads.map(|t| t.join().unwrap());
    
    assert_eq!(CELL.get().unwrap(), "Aufwendig...");
}
