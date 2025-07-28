// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num // G: it has an ";" at the end which was not needed in ruturning.
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
