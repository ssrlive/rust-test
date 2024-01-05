use std::{
    sync::{Arc, Mutex},
    thread,
};

const CONST: i32 = 42;

fn main() {
    let global = Arc::new(Mutex::new(0));
    let global2 = global.clone();
    let handle = thread::spawn(move || {
        let mut data = global2.lock().unwrap();
        *data += 1;
    });
    handle.join().unwrap();
    println!("global = {}", *global.lock().unwrap());

    let global3 = global.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..CONST {
            let mut data = global3.lock().unwrap();
            *data += 1;
        }
    });
    handle2.join().unwrap();
    println!("global = {}", *global.lock().unwrap());
}
