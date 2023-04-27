use rand::Rng;
use std::{sync::{Mutex, Condvar}, collections::VecDeque, thread, time};

#[macro_use]
extern crate lazy_static;

static CAR_SLOTS: i32 = 4;

lazy_static! {
    static ref boarded_queue: (Mutex<i32>, Condvar) = (Mutex::new(0), Condvar::new());
    static ref unboarded_queue: (Mutex<i32>, Condvar) = (Mutex::new(0), Condvar::new());
    static ref loaded: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
    static ref unloaded: (Mutex<bool>, Condvar) = (Mutex::new(true), Condvar::new());
}

fn load(id: i32){
	println!("Ride will begin, time to load!\n");
	println!("This car's capacity is {}\n", CAR_SLOTS);
	thread::sleep(time::Duration::from_secs(2));
}

fn run(id: i32){
	println!("The car is full, time to ride!\n");
	thread::sleep(time::Duration::from_secs(2));
	println!(":: The car is now riding the roller coaster!\n");
	thread::sleep(time::Duration::from_secs(5));
}
fn unload(id: i32){
	println!(":: The ride is over, time to unload!\n");
	thread::sleep(time::Duration::from_secs(2));
}
fn board(id: i32){
    let p_boarded = 0;
	println!(":: {} passengers have boarded the car...\n", p_boarded);
	thread::sleep(time::Duration::from_secs(2));
}

fn unboard(id: i32){
    let p_unboarded = 0;
	println!(":: {} passengers have unboarded the car...\n", p_unboarded);
	thread::sleep(time::Duration::from_secs(2));
}

fn randon_number() -> u64 {
    let mut randon = rand::thread_rng();
    randon.gen_range(1..5)
}

fn passenger_thread() {

}

fn car_thread() {
    
}

fn main() {
    let mut rng = rand::thread_rng();
    let passengers_number = rng.gen_range((6 * CAR_SLOTS)..100);
    // Arc: share memory with all threads and mutex turn data lockable

    let mut handles: Vec<thread::JoinHandle<_>> = vec![];
    // let mut cars = vec![];

    for car_id in 1..6 + 1 {
        println!("car ID: {:?}", car_id);

        let handle = thread::spawn(move || loop {
            load(car_id);
            run(car_id);
            thread::sleep(time::Duration::from_secs(randon_number()));
            unload(car_id);
        });
        handles.push(handle);
    };

    for person_id in 0..passengers_number {
        thread::spawn(move || {
            loop {
                let (lock, cvar) = &*loaded;
                let mut car_loaded = lock.lock().unwrap();

                while !*car_loaded {
                    car_loaded = cvar.wait(car_loaded).unwrap();
                };               

            }
        });
    }

    loop {}
}
