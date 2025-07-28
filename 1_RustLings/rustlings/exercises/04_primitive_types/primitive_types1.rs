// Booleans (`bool`)

fn main() {
    let is_morning = true;
    let is_evening = !is_morning; // G: very imp we decleared is_eve opposite fo is_morning.
    if is_morning {
        println!("Good morning!");
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // let …
    if is_evening {
        println!("Good evening!");
    }
}
