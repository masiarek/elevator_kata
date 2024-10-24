#![warn(clippy::all, clippy::pedantic)]
mod building;
mod floor;
mod person;
mod tests;
mod the_lift;
use rand::Rng;

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    vec![]
}

fn main() {
    println!("Lift simulation program - run 'cargo test.'");
    let mut vec = vec![];
    for _ in 0..4 {
        let person = person::Person::new();
        vec.push(person);
    }

    for e in vec.iter() {
        println!(
            "Person: {}, Requested Floor: {}",
            e.id, e.requested_floor_number
        );
    }
}
