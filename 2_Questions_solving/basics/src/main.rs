use core::hash;

use basics::functions::create_user_from_data::users_data;
use basics::functions::evenodd::{
    difference_between_same_name_types, even_or_odd, even_or_odd_with_reference,
};
use basics::functions::voter_eligiblity::return_eligiblity;
use basics::program_one::event::Event;

/* Y: 1. Odd Even*/
fn run_even_odd_test() {
    let n: i32 = 12;
    println!(
        " The Same even odd but by passing reference in function so {n} is {} number ",
        even_or_odd_with_reference(&n).1
    );
    println!(" The number {n} is {} ", even_or_odd(n).1);
}

/* Y: 2. Eligibility Test*/
fn run_eligibliligy_tests() {
    let mut all_people = users_data();

    for i in &mut all_people {
        i.is_eligibie = return_eligiblity(i.eligibility_age, i.age);
        println!("{:?}", i);
    }
}

/* TAG: EXTRAS*/
fn _run_other_dependency_functions() {
    difference_between_same_name_types();
}

fn main() {
    run_even_odd_test();
    run_eligibliligy_tests();

    // run_other_dependency_functions();
}
