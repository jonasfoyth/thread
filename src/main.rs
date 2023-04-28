use rand::Rng;
use std::{
    sync::{Condvar, Mutex},
    thread,
    time,
};

#[macro_use]
extern crate lazy_static;

static CAR_NUMBER: i32 = 6;
static CAR_SLOTS: i32 = 4;

lazy_static! {
    static ref BOARD_QUEUE: Mutex<i32> = Mutex::new(0);
    static ref UNBOARD_QUEUE: Mutex<i32> = Mutex::new(0);
    static ref ALL_BOARDED: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
    static ref ALL_UNBOARDED: (Mutex<bool>, Condvar) = (Mutex::new(true), Condvar::new());
    static ref ALL_RUNNED: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
    static ref BOARDED_COUNTER: Mutex<i32> = Mutex::new(0);
}

fn load(id: i8) {
    println!("\n:: Ride will begin, time to load!");
    println!("\n:: Car {} capacity is {}", id, CAR_SLOTS);

    let mut queue = BOARD_QUEUE.try_lock().unwrap();

    for _ in 0..CAR_SLOTS {
        (*queue) += 1;
    };

    drop(queue);

}

fn run(id: i8) {
    let (run_lock, run_cvar) = &*RUNNED;
    if let Ok(mut run) = run_lock.try_lock() {
        (*run) = false;
        run_cvar.notify_all();
        drop(run);
    };

    println!("\n:: The car {} is full, time to ride!", id);
    thread::sleep(time::Duration::from_secs(2));
    println!(":: The car {} is now riding the roller coaster!", id);
    thread::sleep(time::Duration::from_secs(5));
}
fn unload(id: i8) {
    println!("\n:: The car {} ride is over, time to unload!", id);
    thread::sleep(time::Duration::from_secs(2));

    if let Ok(mut queue) = UNBOARD_QUEUE.try_lock() {
        for _ in 0..CAR_SLOTS {
            (*queue) += 1;
        };
    
        drop(queue);
    };

}
fn board(id: i32) {

    let queue = BOARD_QUEUE.lock().unwrap();

    if let Ok(mut passengers_boarded) = BOARDED_COUNTER.lock() {
        if (*passengers_boarded) < (*queue) {
            println!("\n:: passenger {} have boarded the car...", id);
            (*passengers_boarded) += 1;
            println!(":: boarded in car {:?}", (*passengers_boarded));
        } else if (*passengers_boarded) == (*queue) {
            let (all_lock, all_cvar) = &*ALL_BOARDED;
            let mut all_boarded = all_lock.lock().unwrap();
            (*all_boarded) = true;
            all_cvar.notify_all();
            drop(all_boarded);

            let (all_un_lock, all_un_cvar) = &*ALL_UNBOARDED;
            let mut all_unboarded = all_un_lock.lock().unwrap();
            (*all_unboarded) = false;
            all_un_cvar.notify_all();
            drop(all_unboarded);
        }
        drop(queue);
        drop(passengers_boarded);
    };
    // println!("\n:: end");

}

fn unboard(id: i32) {
    let queue = UNBOARD_QUEUE.lock().unwrap();

    if let Ok(mut passengers_boarded) = BOARDED_COUNTER.try_lock() {
        if (*passengers_boarded) >= (*queue) {
            println!("\n:: passenger {} have unboarded the car...", id);
            (*passengers_boarded) -= 1;
            println!(":: boarded in car {:?}", (*passengers_boarded));
        } else if (*passengers_boarded) == (*queue) {

            let (all_un_lock, all_un_cvar) = &*ALL_UNBOARDED;
            let mut all_unboarded = all_un_lock.lock().unwrap();
            (*all_unboarded) = true;
            all_un_cvar.notify_all();
            drop(all_unboarded);

            let (all_lock, _all_cvar) = &*ALL_BOARDED;
            let mut all_boarded = all_lock.try_lock().unwrap();
            (*all_boarded) = false;
            all_un_cvar.notify_all();
            drop(all_boarded);
        };
    };
    
    drop(queue);
}

fn passenger_thread(passenger_id: i32) {
    loop {
        board(passenger_id);
        let (runned_lock, cvar) = &*RUNNED;
        let mut car_runned = runned_lock.lock().unwrap();

        while !(*car_runned) {
            println!("\ncar_runned: {}", car_runned);
            car_runned = cvar.wait(car_runned).unwrap();
        };

        println!("\nrunned {}", passenger_id);

        // unboard(passenger_id);

    }
}

fn car_thread(car_id: i8) {
    print!("\nCAR ID: {:}\n", car_id);
    loop {
        load(car_id);
        let (passengers_boarded, cvar) = &*ALL_BOARDED;
        let mut boarded = passengers_boarded.lock().unwrap();

        while !*boarded {
            boarded = cvar.wait(boarded).unwrap();
        };

        run(car_id);

        let (run_lock, run_cvar) = &*RUNNED;
        let mut run = run_lock.lock().unwrap();
        (*run) = true;
        run_cvar.notify_all();

        drop(run);

        loop {}

        // unload(car_id);

        // let (passengers_unboarded, cvar) = &*ALL_UNBOARDED;
        // let mut unboarded = passengers_unboarded.lock().unwrap();
        // println!("\nunboarded: {}", unboarded);
        // while !*unboarded {
        //     unboarded = cvar.wait(unboarded).unwrap();
        // };
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let passengers_number = rng.gen_range((6 * CAR_SLOTS)..100);
    println!("\nPassengers number: {:?}", passengers_number);

    for car_id in 1..6 + 1 {
        println!("car ID: {:?}", car_id);
        thread::spawn(move || loop {
            car_thread(car_id);
        });
    }

    for person_id in 1..passengers_number {
        thread::spawn(move || {
            passenger_thread(person_id);
        });
    }

    loop {}
}
