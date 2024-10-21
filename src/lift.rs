#[derive(Debug)]
enum ButtonDirection {
    Up,
    Down,
}
#[derive(Debug)]
struct Lift {
    button: ButtonDirection,
    lift_max_capacity: u8,
    current_position: u16,
}
//
// impl Lift {
//     fn new (-> Self{
//
//     }
// }