use crate::person::Person;

struct Floor {
    // folks - persons waiting for the Lift to arrive at the Floor
    // represented as a number in a vector (array?)
    person: Vec<Person>,
}
