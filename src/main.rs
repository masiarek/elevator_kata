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
    println!("Lift simulation program. \nRun 'cargo test.'");

    let person = person::Person::new(1, 5);
    println!(
        "Person: {}, Requested Floor: {}",
        person.person, person.requested_floor_number
    );
}
