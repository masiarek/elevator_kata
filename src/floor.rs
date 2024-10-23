use crate::person::Person;

pub struct Floor {
    floor_number: u16,
    person: Person, // we need a queue - vector or array?
                    // persons are waiting for the Lift to arrive at the Floor
                    // represented as a number in a vector (array?)
}
