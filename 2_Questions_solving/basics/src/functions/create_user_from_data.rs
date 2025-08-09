// use basics::functions::create_person::{Person_eligibility, create_person};
// DX: the above code will not work because they are maintain by "lib.rs" so we will go to lib.rs first.
use super::create_person::{Person_eligibility, create_person}; // G: this will work.

/* Y:
 *    1. Here we use super:: to get to lib.rs
 *    2. create_person is the file which contain both
 *               a. Person_eligibility
 *               b. create_person (structs)
 */

pub fn users_data() -> Vec<Person_eligibility> {
    let names: [&str; 7] = [
        "Mukul", "Kaiwalya", "Shruti", "Manu", "Aditya", "Amey", "Aaradhya",
    ];
    let ages: [u8; 7] = [28, 21, 26, 23, 18, 6, 12];

    let mut all_people: Vec<Person_eligibility> = Vec::new();
    for (name, age) in names.iter().zip(ages.iter()) {
        all_people.push(create_person(name, *age));
    }
    all_people
}
