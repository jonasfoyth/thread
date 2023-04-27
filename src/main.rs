use rand::Rng;
use std::{sync::{Mutex, Condvar}, thread, time};

#[macro_use]
extern crate lazy_static;

static CAR_NUMBER: i32 = 6;
static CAR_SLOTS: i32 = 4;
static CAPACITY: i32 = 24;

lazy_static! {
    static ref BOARD_QUEUE: (Mutex<i32>, Condvar) = (Mutex::new(0), Condvar::new());
    static ref UNBOARD_QUEUE: (Mutex<i32>, Condvar) = (Mutex::new(0), Condvar::new());
    static ref ALL_BOARDED: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
    static ref ALL_UNBOARDED: (Mutex<bool>, Condvar) = (Mutex::new(true), Condvar::new());
    static ref BOARDED_COUNTER: Mutex<i32> = Mutex::new(0);
}

fn load(id: i8){
	println!("\nRide will begin, time to load!\n");
	println!("This car's capacity is {}\n", CAR_SLOTS);
	thread::sleep(time::Duration::from_secs(2));
}

fn run(id: i8){
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
	// println!(":: passenger {} have boarded the car...\n", id);
    let mut passengers_boarded = BOARDED_COUNTER.lock().unwrap();
    (*passengers_boarded) += 1;
    println!(":: passengers boarded: {}\n", (*passengers_boarded));

    drop(passengers_boarded);
	thread::sleep(time::Duration::from_secs(2));
}

fn unboard(id: i32){
    let mut passengers_boarded = BOARDED_COUNTER.lock().unwrap();
    (*passengers_boarded) -= 1;
    println!(":: passengers boarded: {}\n", (*passengers_boarded));
}

fn randon_number() -> u64 {
    let mut randon = rand::thread_rng();
    randon.gen_range(1..5)
}

fn passenger_thread(passenger_id: i32) {
    loop {
        let (board_lock, cvar) = &*BOARD_QUEUE;

        if let Ok(mut car_boarded) = board_lock.lock() {
            while (*car_boarded) < CAPACITY {
                println!("\n passenger {:?}", passenger_id);
                println!("\n car_boarded {:?}", (*car_boarded));
                car_boarded = cvar.wait(car_boarded).unwrap();
            };         
    
            board(passenger_id);
    
            drop(car_boarded);
        }
    
        println!("\nPASS TESTE 1");

        let passengers_boarded = BOARDED_COUNTER.lock().unwrap();

        if (*passengers_boarded) == CAPACITY {
            println!("\nPASS TESTE 2");
            let (allboard_lock, all_cvar) = &*ALL_BOARDED;
            let mut all_boarded = allboard_lock.lock().unwrap();
            (*all_boarded) = true;
            all_cvar.notify_one();
            drop(all_boarded);
        };

        // drop(passengers_boarded);

        // println!("PASS TESTE 3");

        // let (unboard_lock, cvar) = &*UNBOARD_QUEUE;
        // let mut car_unboarded = unboard_lock.lock().unwrap();

        // while (*car_unboarded) == 0 {
        //     car_unboarded = cvar.wait(car_unboarded).unwrap();
        // }; 

        // drop(car_unboarded);
        // unboard(passenger_id);

    }
}

fn car_thread(car_id: i8) {
    print!("\nCAR ID: {:}", car_id);
    loop {
        load(car_id);

        let (board_queue, quee_cvar) = &*BOARD_QUEUE;

        let mut queue: std::sync::MutexGuard<i32> = board_queue.lock().unwrap();
        print!("\nCAR QUEUE: {:}", (*queue));

        for _ in 0..CAR_SLOTS {
            (*queue) += 1; 
        };

        quee_cvar.notify_one();

        drop(queue);

        let (passengers_boarded, cvar) = &*ALL_BOARDED;
        let mut boarded = passengers_boarded.lock().unwrap();

        while !*boarded {
            boarded = cvar.wait(boarded).unwrap();
        };         

        run(car_id);
        print!("\nCAR TESTE 3");

        // let (unboard_queue, unquee_cvar) = &*UNBOARD_QUEUE;
        // let mut un_queue: std::sync::MutexGuard<i32> = unboard_queue.lock().unwrap();

        // for _ in 0..CAR_SLOTS {
        //     (*un_queue) -= 1; 
        // };

        // drop(un_queue);
        // unquee_cvar.notify_one();


    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let passengers_number = rng.gen_range((6 * CAR_SLOTS)..100);
    println!("\nPassengers number: {:?}", passengers_number);

    // Arc: share memory with all threads and mutex turn data lockable

    let mut handles: Vec<thread::JoinHandle<_>> = vec![];
    // let mut cars = vec![];

    for car_id in 1..6 + 1 {
        println!("car ID: {:?}", car_id);
        let handle = thread::spawn(move || loop {
            car_thread(car_id);

        });
        handles.push(handle);
    };

    for person_id in 0..passengers_number {
        thread::spawn(move || {
            passenger_thread(person_id);
        });
    }

    loop {}
}
