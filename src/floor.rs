use crate::person::Person;

pub struct Floor {
    floor_number: u16,
    persons_on_the_floor: Vec<Person>,
}

// cheating - using chatGPT ...
// the floor has a queue of persons waiting for the lift.
impl Floor {
    pub fn new(floor_number: u16) -> Floor {
        Floor {
            floor_number,
            persons_on_the_floor: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons_on_the_floor.push(person);
    }

    pub fn get_few_persons_on_the_floor(&self) -> &Vec<Person> {
        &self.persons_on_the_floor
    }
}
