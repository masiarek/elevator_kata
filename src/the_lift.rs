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
    lift_no: u8,
}
