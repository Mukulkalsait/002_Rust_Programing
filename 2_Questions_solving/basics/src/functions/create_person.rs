#[derive(Debug)]
pub struct Person_eligibility {
    pub name: String,
    pub age: u8,
    pub eligibility_age: u8,
    pub is_eligibie: bool,
}

pub fn create_person(name: &str, a: u8) -> Person_eligibility {
    Person_eligibility {
        name: name.to_string(),
        age: a,
        eligibility_age: 18,
        is_eligibie: false,
    }
}
