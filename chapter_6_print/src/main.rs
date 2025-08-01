fn main() {
    print_1();
    print_2();
    print_3();
    print_4();
    print_5();
}

// G: We can treat arguments like its an Array and print them with index 0,1,2,3 & ..n .
fn print_1() {
    println!("_________________________");
    println!(
        "a: {0}, b: {1} anather a: {0} and last one this time {2}",
        "1_first", "2_second", "3_third"
    );
}

// G: We can Name arguments.
fn print_2() {
    println!("  _________________________");
    println!(
        " |sub:{subject}\n |verb:{verb}\n |obj:{object}\n |int: {other_type}\n |char_type: {char_type}",
        object = "The object section.",
        subject = "The subject section.",
        verb = "The verb section.",
        other_type = 24,
        char_type = '😎'
    );
}

// Y: We can treat arguments like its an Array and print them with index 0,1,2,3 & ..n .
fn print_3() {
    println!("  _________________________");
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    println!("this is binary of number 13: {:b}", 13);
    format!("string binary is giving Error");
}

// Y: We can treat arguments like its an Array and print them with index 0,1,2,3 & ..n .
fn print_4() {
    println!("  _________________________");
}

// Y: We can treat arguments like its an Array and print them with index 0,1,2,3 & ..n .
fn print_5() {
    println!("  _________________________");
}
