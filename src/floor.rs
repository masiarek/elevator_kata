use crate::person::Person;

pub struct Floor {
    floor_number: u16,
    person: Person, // we need a queue - vector or array?
                    // persons are waiting for the Lift to arrive at the Floor
                    // represented as a number in a vector (array?)
}

// the floor has a queue of persons waiting for the lift?
// person has a floor they want to go to?

// What is created first:
// a person with a requested floor number
// a flor - triggering persons to be created with a requested floor number
