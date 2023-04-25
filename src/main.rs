use rand::Rng;
use static_init::dynamic;
use std::{collections::VecDeque, thread, time};

#[derive(Clone, Debug)]

struct Car {
    id: Option<i32>,
    free_slots: Option<i32>,
    running: bool,
    can_board: bool,
}

#[derive(Clone, Debug)]

struct Passenger {
    id: Option<i32>,
    can_load: bool,
    boarded: bool,
}

impl Passenger {
    fn initalize(number: i32) {
        let mut free_passengers = PASSANGERS.write();
        for id in 0..number {
            (*free_passengers).push_back(Passenger {
                id: Some(id),
                can_load: false,
                boarded: false,
            });
        }

        drop(free_passengers);
    }
}

impl Car {
    fn initalize(id: Option<i32>, slots: Option<i32>) -> Self {
        Car {
            id: id,
            free_slots: slots,
            running: false,
            can_board: false,
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
            running: false,
            can_board: false,
        }
    }
}

impl Default for Passenger {
    fn default() -> Self {
        Passenger {
            id: None,
            can_load: false,
            boarded: false,
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
static mut PASSANGERS: VecDeque<Passenger> = VecDeque::new();

fn car_load(car_id: i8) {
    let mut free_passengers = PASSANGERS.write();

    println! {"\n:: Free passengers: {:?}", (*free_passengers)};

    let mut car_info = match car_id {
        1 => CAR1.write(),
        2 => CAR2.write(),
        3 => CAR3.write(),
        4 => CAR4.write(),
        5 => CAR5.write(),
        6 => CAR6.write(),
        _ => return,
    };

    println! {"\n:: Car Status - before board"};
    println!(":: id: {:?}", (*car_info).id,);
    println!(":: free slots: {:?}", (*car_info).free_slots);

    if (*free_passengers).len() > 0 {
        if let Some(slots) = (*car_info).free_slots {
            (*car_info).can_board = true;
            // for _ in 1..slots {
            //     let mut passenger : Passenger= (*free_passengers).swap_remove(0);
            //     passenger.can_board = true;
            //     (*free_passengers).push(passenger);
            // }
        } else {
            (*car_info).can_board = false;
        }
    };

    (*car_info).free_slots.clone_from(&None);

    println! {"\n:: Car Status - after board"};
    println!(":: id: {:?}", (*car_info).id,);
    println!(":: free slots: {:?}", (*car_info).free_slots);
    println! {":: after board Free passengers: {:?}", (*free_passengers)};
    drop(car_info);
    drop(free_passengers);
}

fn car_unload(car_id: i8) {
    let mut car_info = match car_id {
        1 => CAR1.write(),
        2 => CAR2.write(),
        3 => CAR3.write(),
        4 => CAR4.write(),
        5 => CAR5.write(),
        6 => CAR6.write(),
        _ => return,
    };

    (*car_info).free_slots.clone_from(&Some(CAR_SLOTS));
    (*car_info).can_board.clone_from(&false);
    drop(car_info);

    // let mut free_passengers = PASSANGERS.write();
    // (*free_passengers).iter().
    // (*PASSANGERS).push();
    // println! {"\n:: Free passengers: {:?}", (*free_passengers)};
    // drop(free_passengers);
}

fn car_run(car_id: i8) {
    println!(":: \nRunning car id {:?}", car_id);

    let mut car_info = match car_id {
        1 => CAR1.write(),
        2 => CAR2.write(),
        3 => CAR3.write(),
        4 => CAR4.write(),
        5 => CAR5.write(),
        6 => CAR6.write(),
        _ => return,
    };

    if let Some(free_slots) = (*car_info).free_slots {
        if free_slots == 0 {
            (*car_info).running = true
        }
    }

    drop(car_info)
}

fn passengers_board(passangers: i32) -> i32 {
    passangers
}

fn passengers_unboard(passangers: i32) -> i32 {
    passangers
}

fn randon_number() -> u64 {
    let mut randon = rand::thread_rng();
    randon.gen_range(1..5)
}

fn main() {
    let mut rng = rand::thread_rng();
    let passengers_number = rng.gen_range((6 * CAR_SLOTS)..100);
    // Arc: share memory with all threads and mutex turn data lockable
    Passenger::initalize(passengers_number);

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

    for person_id in 0..passengers_number {
        thread::spawn(move || loop {
            let mut passangers = PASSANGERS.write();
            println!("\n:: person id: {}", person_id);

            if let Some(passenger) = (*passangers)
                .iter()
                .find(|&person| person.id == Some(person_id))
            {
                println!(":: passenger : {:?}", passenger);

                if passenger.can_load {
                    passengers_board(passenger.id.unwrap());
                } else {
                    passengers_unboard(passenger.id.unwrap());
                };
            }


            // println!("");
            drop(passangers);
        });
    }

    loop {}
}
