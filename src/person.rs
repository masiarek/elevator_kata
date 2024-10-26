//! This module contains the Person struct and its implementation.
//! Link to the URL: <https://www.codewars.com/kata/58905bfa1decb981da00009e>
//!
//! This is crate level doc...
/*
- People are in "queues" that represent their order of arrival to wait for the Lift
- All people can press the UP/DOWN Lift-call buttons
- Only people going the same direction as the Lift may enter it
- Entry is according to the "queue" order, but those unable to enter do not block
  those behind them that can
- If a person is unable to enter a full Lift, they will press the UP/DOWN Lift-call button
  again after it has departed without them
 */
/// learn the difference between module and crate documentation - this is Person...
///
/// New paragraph test. Why is this line not printing in the doc...
/// another line.
///
///

#[derive(Debug)]
pub struct Person {
    pub id: usize,
    pub requested_floor_number: u16,
}

impl Person {
    pub fn new(id: usize) -> Self {
        Person {
            id,
            requested_floor_number: 1,
        }
    }
}
// test cases are using simple arrays to represent the queues (with people in them)
// - maybe there is no need to create a struct for the person?
// - use a simple vec with u16 as requested floor number
// the benefit of a person struct
// - store more information about the person
// - easier debugging - which person got in/out - on which floor
