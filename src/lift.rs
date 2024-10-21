#[derive(Debug)]
enum ButtonDirection {
    Up,
    Down,
}
#[derive(Debug)]
pub struct Lift {
    button: ButtonDirection,
    lift_max_capacity: u8,
    current_position: u16,
    lift_no: u8,
}
//
// impl Lift {
//     fn new (-> Self{
// testerb test test test again
//     }
// }
