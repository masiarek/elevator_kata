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
