use std::{thread, time};


fn main() {
    thread::spawn(|| {
        println!("Hello, world from thread!");
        thread::sleep(time::Duration::from_millis(1));
    }); 
    println!("Hello, world from main thread!");
}
