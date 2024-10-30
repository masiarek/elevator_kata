/*
- The Lift only goes up or down.
- Each floor has both UP and DOWN Lift-call buttons
  (except top and first floor which have only DOWN and UP respectively)
- The Lift never changes direction until there are no more people
  wanting to get on/off in the direction it is already travelling
- When empty the Lift tries to be smart. For example,
  - If it was going up then it will continue up to collect
    the highest floor person wanting to go down
  - If it was going down then it will continue down to collect
    the lowest floor person wanting to go up
- The Lift has a maximum capacity of people
- When called, the Lift will stop at a floor even if it is full,
  although unless somebody gets off nobody else can get on!
- If the lift is empty, and no people are waiting, then it will return to the ground floor
 */
#[derive(Debug)]
pub enum ButtonDirection {
    Up,
    Down,
}
#[derive(Debug)]
pub struct Lift {
    button: ButtonDirection,
    capacity: u8, // capacity maximum number of people allowed in the lift
    current_position: u16,
    // passengers: Vec<Person>,
}
//
// fn the_lift(queues, capacity){
//
// }
