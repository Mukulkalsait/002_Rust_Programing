fn main() {
    println!("Hello, world!");

    if_else_understanding();
}

fn if_else_understanding() {
    /* comment
     *  Y:: if-elseif-else
     *  1. Conduction must be EXPLICETYL "BULLEAN".
     *  2. hence -> if (number) {...}  // will not work (number is not bull)
     *  3. "()" are not necessary unless too complex expression.
     */

    // Y: 1. EXPLICETYL BULLEAN:
    let conduction = true;
    let number: u32 = if conduction { 32 } else { 64 }; // Y:  U32 will make sure its NonNegagive.

    // Y: 2 & 3 :
    if number < 50 {
        println!("num is small");
    } else if number == 50 {
        println!("number is equal !!");
    } else {
        println!("Number is alleready greter.");
    }
}
