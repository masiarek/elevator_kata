#![warn(clippy::all, clippy::pedantic)]
mod building;
mod floor;
mod person;
mod tests;
mod the_lift;

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    vec![]
}

fn main() {
    println!("Lift simulation program - run 'cargo test.'");
    let mut id = 0;

    let mut vec = vec![];
    for _ in 0..4 {
        id += 1;
        let person = crate::person::Person::new(id);
        vec.push(person);
    }

    for person in &vec {
        println!(
            "Person: {}, Requested Floor: {}",
            person.id, person.requested_floor_number
        );
    }
}
