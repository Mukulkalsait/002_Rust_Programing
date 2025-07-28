// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; // G: Mutability.
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
