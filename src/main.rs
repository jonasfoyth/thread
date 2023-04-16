use rand::Rng;
use std::sync::Mutex;
use std::{thread, time};

// C = Car passengers number
// n = n passangers
// m = car numbers = 6

// C > 0
// n >= C+1
// m > 0 e m < 6
// m*C < n || n > m*C

// passengers canot board ultil car invoke load

fn car_load() {}

fn car_unload() {}

fn car_run() {}

fn passenger_board() {}

fn passenger_unboard() {}

fn main() {
    let mut rng = rand::thread_rng();

    let m = Mutex::new(6);
    let C = Mutex::new(rng.gen_range(1..10));
    let n = Mutex::new(1);

    let mut handles = vec![];

    let handle = thread::spawn(|| {
        println!("Hello, world from thread!");
        thread::sleep(time::Duration::from_millis(1));
    });

    handles.push(handle);

    loop {
        println!("waiting car");
    }
}
