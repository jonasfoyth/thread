use rand::Rng;
use static_init::dynamic;
use std::{thread, time};

#[derive(Clone, Debug)]

struct Car {
    id: Option<i32>,
    free_slots: Option<i32>,
    running: bool,
}

#[derive(Clone, Debug)]

struct Passenger {
    can_board: bool,
}

impl Car {
    fn initalize(id: Option<i32>, slots: Option<i32>) -> Self {
        Car {
            id: id,
            free_slots: slots,
            running: false
        }
    }

    fn check_space(self) -> bool {
        self.free_slots != None && self.free_slots != Some(0)
    }

    fn can_board(self) -> bool {
        !self.running
    }
}

impl Default for Car {
    fn default() -> Self {
        Car {
            id: None,
            free_slots: Some(CAR_SLOTS),
            running: false
        }
    }
}

impl Default for Passenger {
    fn default() -> Self {
        Passenger {
            can_board: false
        }
    }
}

static CAR_SLOTS: i32 = 4;

#[dynamic(drop)]
static mut CAR1: Car = Car::default();
#[dynamic(drop)]
static mut CAR2: Car = Car::default();
#[dynamic(drop)]
static mut CAR3: Car = Car::default();
#[dynamic(drop)]
static mut CAR4: Car = Car::default();
#[dynamic(drop)]
static mut CAR5: Car = Car::default();
#[dynamic(drop)]
static mut CAR6: Car = Car::default();
#[dynamic(drop)]
static mut PASSANGERS: Vec<Passenger> = Vec::new();


fn car_load(car_id: i8) {
    let mut free_passengers = PASSANGERS.write();

    println! {"\n:: Free passengers: {}", (*free_passengers)};

    let car_info = match car_id {
        1 => CAR1.write(),
        2 => CAR2.write(),
        3 => CAR3.write(),
        4 => CAR4.write(),
        5 => CAR5.write(),
        6 => CAR6.write(),
    };

    println! {"\n:: Car Status - before board"};
    println!(":: id: {:?}", (*car_info).id,);
    println!(":: free slots: {:?}", (*car_info).free_slots);

    if (*car_info).clone().check_space() {
        if (*free_passengers).len() > 0 {
            (*car_info).free_slots.clone_from(&None);
            (*free_passengers) -= passenger_board(CAR_SLOTS);
        };

        println! {"\n:: Car Status - after board"};
        println!(":: id: {:?}", (*car_info).id,);
        println!(":: free slots: {:?}", (*car_info).free_slots);
        println! {":: after board Free passengers: {}", (*free_passengers)};
        drop(car_info);
        drop(free_passengers);
    }
}

fn car_unload(car_id: i8) {
    let mut free_passengers = PASSANGERS.write();

    println! {"\n:: Free passengers: {}", (*free_passengers)};

    let car_info = match car_id {
        1 => CAR1.write(),
        2 => CAR2.write(),
        3 => CAR3.write(),
        4 => CAR4.write(),
        5 => CAR5.write(),
        6 => CAR6.write(),
    };

    (*car_info).free_slots.clone_from(&Some(CAR_SLOTS));
    drop(car_info);
}


fn car_run(car_id: i8) {
    println!(":: \nRunning car id {:?}", car_id);

    let car_info = match car_id {
        1 => CAR1.write(),
        2 => CAR2.write(),
        3 => CAR3.write(),
        4 => CAR4.write(),
        5 => CAR5.write(),
        6 => CAR6.write(),
    };


    if let Some(free_slots) = (*car_info).free_slots {
        if free_slots  == 0 {
            (*car_info).running = true
        }
    }
    
    drop(car_info)

}

fn passenger_board(passangers: i32) -> i32 {
    passangers
}

fn passenger_unboard(passangers: i32) -> i32 {
    passangers
}

fn randon_number() -> u64 {
    let mut randon = rand::thread_rng();
    randon.gen_range(1..5)
}

fn main() {
    let mut rng = rand::thread_rng();

    let passangers = PASSANGERS.write();
    *passangers = rng.gen_range((6 * CAR_SLOTS)..100); // minimum passangers number fixed to (m*C)
    // Arc: share memory with all threads and mutex turn data lockable

    let mut handles: Vec<thread::JoinHandle<_>> = vec![];
    // let mut cars = vec![];

    for car_id in 1..6 + 1 {
        println!("car ID: {:?}", car_id);

        let handle = thread::spawn(move || loop {
            car_load(car_id);
            car_run(car_id);
            thread::sleep(time::Duration::from_secs(randon_number()));
            car_unload(car_id);
        });
        handles.push(handle);
    }

    for car_id in 1..(*passangers) {
        thread::spawn(move || loop {
            // println!("");

        });
    }

    drop(passangers);

    loop {
        passangers
    }
}
