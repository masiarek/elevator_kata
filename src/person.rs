/// This module contains the Person struct and its implementation (rargo test --doc)
/// Link to the URL: https://www.codewars.com/kata/58905bfa1decb981da00009e
/// The link works OK in the code itself - but not in the browser
/*
- People are in "queues" that represent their order of arrival to wait for the Lift
- All people can press the UP/DOWN Lift-call buttons
- Only people going the same direction as the Lift may enter it
- Entry is according to the "queue" order, but those unable to enter do not block
  those behind them that can
- If a person is unable to enter a full Lift, they will press the UP/DOWN Lift-call button
  again after it has departed without them
 */
pub struct Person {
    pub person: u16,
    pub requested_floor_number: u16,
}

impl Person {
    pub fn new(person: u16, requested_floor_number: u16) -> Self {
        Self {
            person,
            requested_floor_number,
        }
    }
}

// fn number_of_people(floor: Floor){
// let num_of_people_of_floor = rand::thread_rng().gen_range(1..5);
// println!("Number of people on floor {}", num_of_people_of_floor)
