use rand::Rng;
use std::{
    sync::Mutex,
    thread, time,
};

#[macro_use]
extern crate lazy_static;

static CAR_NUMBER: i8 = 6;
static CAR_SLOTS: i8 = 4;

lazy_static! {
    static ref ALL_BOARDED: Mutex<i32> = Mutex::new(0);
    static ref CAR_FULL: Mutex<bool> = Mutex::new(false);
    static ref LOAD: Mutex<bool> = Mutex::new(true);
    static ref CAR_RUNNING: Mutex<Vec<bool>> = Mutex::new(Vec::with_capacity(6));
}

fn runned_status() -> Vec<bool> {
    CAR_RUNNING.lock().unwrap().to_vec()
}

fn runned_upate(state: bool, pos: i8) {
    CAR_RUNNING.lock().unwrap()[(pos - 1) as usize] = state;
}

fn car_status() -> bool {
    CAR_FULL.lock().unwrap().clone()
}

fn car_upate(state: bool) {
    CAR_FULL.lock().unwrap().clone_from(&state);
}

fn load_status() -> bool {
    LOAD.lock().unwrap().clone()
}

fn load_update(state: bool) {
    LOAD.lock().unwrap().clone_from(&state);
}

fn load(id: i8) {
    println!("\n:: Car {} is ready to load", id);
    load_update(true);
}

fn run(id: i8) -> bool {
    println!("\n:: The car {} is full, time to ride!", id);
    thread::sleep(time::Duration::from_secs(5));
    println!(":: The car {} is now riding the roller coaster!", id);
    true
}

fn unload(id: i8) {
    println!("\n:: Car {} is ready to unload", id);
    load_update(false);
    thread::sleep(time::Duration::from_secs(2));
}

fn board(id: i8) -> (bool, bool) {
    thread::sleep(time::Duration::from_secs(1));
    if let Ok(mut queue) = ALL_BOARDED.lock() {
        println!("\n:: Persons boarded: {} ", (*queue));
        if (*queue) == 24 {
            println!(":: Person {} not boarded", id);
            return (true, true);
        } else {
            (*queue) += 1;
            println!(":: Person {} boarded", id);
            return (true, false);
        };
    } else {
        return (false, false);
    }
}

fn unboard(id: i8) -> bool {
    thread::sleep(time::Duration::from_secs(2));
    if let Ok(mut queue) = ALL_BOARDED.lock() {
        println!("\n:: Persons boarded: {} ", (*queue));
        if (*queue) != 0 {
            (*queue) -= 1;
            println!("\n:: Person {} unboarded", id);
        } else if (*queue) == 0 {
            car_upate(false);
        };
        drop(queue);
    };
    return false;
}

fn main() {
    println!("\n:: Ride will begin, time to load!");
    let mut rng = rand::thread_rng();
    let passengers_number = rng.gen_range((6 * CAR_SLOTS)..100);
    println!("\n:: Passengers number: {:?}", passengers_number);

    for car_id in 1..(CAR_NUMBER + 1) {
        CAR_RUNNING.lock().unwrap().push(false);
        thread::spawn(move || {
            let mut runned = false;
            // runned_upate(false, car_id);
            loop {
                if runned {
                    if load_status() != false {
                        unload(car_id);
                    };
                    if !car_status() {
                        runned_upate(true, car_id);
                        if runned_status().iter().all(|&x| x == true) {
                            runned = false;
                        };
                    };
                } else {
                    if car_status() {
                        runned = run(car_id);
                    } else if load_status() != true {
                        runned_upate(false, car_id);
                        load(car_id);
                    };
                };
            }
        });
    }

    for person_id in 1..passengers_number {
        thread::spawn(move || {
            let mut boarded = false;
            loop {
                if load_status() {
                    // println!("\n:: LOAD status: true");
                    if let Ok(mut car_full) = CAR_FULL.lock() {
                        if !(*car_full) {
                            // println!("PERSON ID: {}", person_id);
                            if !boarded {
                                let status = board(person_id);
                                (*car_full) = status.1;
                                boarded = status.0;
                            };
                        } else {
                            // println!("\n:: person {} waiting next board", person_id);
                        }
                        drop(car_full);
                    }
                } else {
                    // println!("\n:: LOAD status: false");
                    // println!("\n:: Person {} - boarded: {}", person_id, boarded);
                    if car_status() {
                        // println!("car stats: {}", car_status());
                        if boarded {
                            boarded = unboard(person_id);
                            thread::sleep(time::Duration::from_secs(2));
                        };
                    }
                }
                thread::sleep(time::Duration::from_secs(2));
            }
        });
    }

    loop {}
}
