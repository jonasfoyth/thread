use lazy_static::__Deref;
use rand::Rng;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread, time,
};

#[macro_use]
extern crate lazy_static;

static CAR_NUMBER: i32 = 6;
static CAR_SLOTS: i32 = 4;
static CAPACITY: i32 = 24;

// lazy_static! {
//     static ref BOARD_QUEUE: Mutex<i32> = Mutex::new(0);
//     static ref UNBOARD_QUEUE: Mutex<i32> = Mutex::new(0);
//     static ref ALL_BOARDED: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
//     static ref ALL_UNBOARDED: (Mutex<bool>, Condvar) = (Mutex::new(true), Condvar::new());
//     static ref RUNNED: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
//     static ref BOARDED_COUNTER: Mutex<i32> = Mutex::new(0);
// }

fn load(id: i8) {
    println!("\n:: Ride will begin, time to load!");
    // println!("\n:: Car {} capacity is {}", id, CAR_SLOTS);
}

fn run(id: i8) {
    // let (run_lock, run_cvar) = &*RUNNED;
    // let mut run = run_lock.try_lock().unwrap();
    // (*run) = false;
    // run_cvar.notify_all();
    // drop(run);

    // println!("\n:: The car {} is full, time to ride!", id);
    // thread::sleep(time::Duration::from_secs(2));
    // println!(":: The car {} is now riding the roller coaster!", id);
    // thread::sleep(time::Duration::from_secs(5));
}
fn unload(id: i8) {
    // println!("\n:: The car {} ride is over, time to unload!", id);
    // thread::sleep(time::Duration::from_secs(2));

    // if let Ok(mut queue) = UNBOARD_QUEUE.try_lock() {
    //     for _ in 0..CAR_SLOTS {
    //         (*queue) += 1;
    //     }

    //     drop(queue);
    // };
}
fn board(id: i32) {
    // println!("\n:: Person {} boarded", id);
    // println!("\n:: end");
}

fn unboard(id: i32) {
    // let queue = UNBOARD_QUEUE.lock().unwrap();

    // if let Ok(mut passengers_boarded) = BOARDED_COUNTER.try_lock() {
    //     if (*passengers_boarded) >= (*queue) {
    //         println!("\n:: passenger {} have unboarded the car...", id);
    //         (*passengers_boarded) -= 1;
    //         println!(":: boarded in car {:?}", (*passengers_boarded));
    //     } else if (*passengers_boarded) == (*queue) {
    //         let (all_un_lock, all_un_cvar) = &*ALL_UNBOARDED;
    //         let mut all_unboarded = all_un_lock.lock().unwrap();
    //         (*all_unboarded) = true;
    //         all_un_cvar.notify_all();
    //         drop(all_unboarded);

    //         let (all_lock, _all_cvar) = &*ALL_BOARDED;
    //         let mut all_boarded = all_lock.try_lock().unwrap();
    //         (*all_boarded) = false;
    //         all_un_cvar.notify_all();
    //         drop(all_boarded);
    //     };
    // };

    // drop(queue);
}

fn main() {
    print!("\n\n:: BEGIN ####################################################\n\n");

    let mut rng = rand::thread_rng();
    let passengers_number = rng.gen_range((6 * CAR_SLOTS)..100);
    println!("\nPassengers number: {:?}", passengers_number);

    let all_boarded: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let running: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let queue_counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let queue_size: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut persons = Vec::with_capacity(passengers_number as usize);

    for car_id in 1..6 + 1 {
        let all_boarded_clone = Arc::clone(&all_boarded);
        let running_clone = Arc::clone(&running);
        let queue_counter_clone = Arc::clone(&queue_counter);
        let queue_size_clone = Arc::clone(&queue_size);

        thread::spawn(move || loop {
            load(car_id);
            let &(ref all_boarded, ref all_cnd) = &*all_boarded_clone;
            let mut guard: std::sync::MutexGuard<_> = all_boarded.lock().unwrap();
            while !*guard {
                println!("car {:?} waiting", car_id);
                guard = all_cnd.wait(guard).unwrap();
            }
            *guard = false;

            drop(guard);
            println!("\ncar ID: {:?} starting", car_id);

            loop {
                println!("\n car {} running", car_id);
                thread::sleep(time::Duration::from_secs(10));
            }

            // let &(ref all_boarded, ref all_cnd) = &*all_boarded_clone;
            // let mut guard: std::sync::MutexGuard<_> = all_boarded.lock().unwrap();
            // while !*guard {
            //     println!("car {:?} waiting", car_id);
            //     guard = all_cnd.wait(guard).unwrap();
            // }
            // *guard = false;

            // drop(guard);
            // unload(car_id);
        });
    }

    for person_id in 1..passengers_number {
        let all_boarded_clone = Arc::clone(&all_boarded);
        let running_clone = Arc::clone(&running);
        let queue_counter_clone = Arc::clone(&queue_counter);
        let queue_size_clone = Arc::clone(&queue_size);
        persons.push(thread::spawn(move || loop {
            // loop {}

            // let &(ref runned_lock, ref cvar) = &*running;
            // let mut car_runned = runned_lock.lock().unwrap();
            let mut passengers_boarded = queue_counter_clone.lock().unwrap();
            // println!(":: passengers {:?}", (*passengers_boarded));
            // println!(":: all slots free {:?}", CAPACITY - (*passengers_boarded));

            if (*passengers_boarded) == CAPACITY {
                // println!("\n unlock run");
                for _ in 0..CAR_NUMBER {
                    let &(ref all_lock, ref all_cvar) = &*all_boarded_clone;
                    let mut all_boarded = all_lock.lock().unwrap();
                    (*all_boarded) = true;
                    all_cvar.notify_all();
                    drop(all_boarded);
                }
            } else if (*passengers_boarded) < CAPACITY {
                board(person_id);
                (*passengers_boarded) += 1;
                // println!(":: boarded in car {:?}", (*passengers_boarded));
            };
            drop(passengers_boarded);

            // let &(ref run_lock, ref run_cnd) = &*running_clone;
            // let mut running: std::sync::MutexGuard<_> = run_lock.lock().unwrap();

            let &(ref run_lock, ref run_cnd) = &*all_boarded_clone;
            let mut guard: std::sync::MutexGuard<_> = run_lock.lock().unwrap();
            while !*guard {
                println!("person {:?} waiting", person_id);
                guard = run_cnd.wait(guard).unwrap();
                thread::sleep(time::Duration::from_secs(2));
            }
            *guard = false;
        }));
    }

    for person in persons {
        person.join().unwrap();
        // println!("{:?}",);
    }
}
