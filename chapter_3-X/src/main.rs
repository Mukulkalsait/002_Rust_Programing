fn main() {
    println!(
        "---------------------------------------------------\n MAIN() :Start\n---------------------------------------------------"
    );
    println!("const can be access BEFORE DECLERATION:{TWO}"); // Y: const can be access BEFORE DECLERATION.

    create_fetch_conts();
    learn_data_types();

    println!(
        "---------------------------------------------------\n MAIN() === END\n---------------------------------------------------"
    );
}

// Y: created CONST outside FN, with CAPITAL leters.
const TWO: u32 = 1 + 1;
fn create_fetch_conts() {
    println!(
        "---------------------------------------------------\nX. TestFunction: create_fetch_conts() === >"
    );
    println!("Accessing Const: {TWO}"); // success
}

fn learn_data_types() {
    println!(
        "---------------------------------------------------\nA. Function: learn_data_types() === >"
    );
    println!(" ===== Data Types 2(S-C)=====");
    println!("-----> A. Scaller DT  BIFC\n-----> A. Compound DT TAF\n");

    /* Y: 2 main data types:
     * A. Scaller Datatypes.
     * B. Compound Datatypes.
     *
     *
     * DX:======== A Scaller ============================================================================================
     *
     * G: Scaller Types.
     *  ---------------
     *  Intiger
     *  Floatpoint number
     *  Boolean
     *  Charecter
     */
    scaller_int();
}

fn scaller_int() {
    println!(
        "---------------------------------------------------\nA.1: Function: scaller_int() === >"
    );
    /* IMP:
     * 1. INT:
     * ----------------------------------------------------
     * \\          \          \         \\                \
     * \\  length  \  signed  \ Unsined \\     Float      \
     * \\          \   +/-    \    +    \\ allways singed \
     * \\-------------------------------------------------\
     * \\  8-bit   \   i8     \   u8    \\    ---         \
     * \\  16-bit  \   i16    \   u16   \\    ---         \
     * \\  32-bit  \   i32    \   u32   \\    i32         \
     * \\  64-bit  \   i64    \   u64   \\    i64         \
     * \\  128-bit \   i128   \   u128  \\    i128        \
     * \\----------\--------------------\\----------------\
     * \\   ARCH   \           architecture depend        \
     * ===================================================
     *
     *  B: ARCHITECTURE = mostyle (32/64)
     * */

    let a = 98_324; // Decimal
    let b = 0xfffa; // Hex
    let c = 0o7735; // Octal
    let _d = 0b1111; // Binery
    let _e = b'A'; // Bite U8 only.
    let f: u8 = 255; // G: intiger oferflow 👇

    println!(
        "a:{}\nb:{}\nc:{}\n=>(The valuse of d and e are not printed cos we used _d and _e to tell compailer that we are not using it and we are doing it on purpose.) \nf:{}",
        a, b, c, f
    );

    /*
     *
     * B: signed vs unisgned :
     * bits   = X 0 1 0 1 0 => here X is signe
     * signed = _ _ _ _ _ _
     * Y:
     * therefore : signed variables are from -128 to 127 ( one place for sign.)
     * unsigned has Intiger OverFlow
     * B: Intiger Overflow:
     *  \--------------------------------------------------------\
     *  \f:u8 = 255 is ( 8 bit unsigned intiger) maxvalue = 255  \
     *  \--------------------------------------------------------\
     *  \                                                        \
     *  \       assigingin it higher value:                      \
     *  \    A. while DEBUG: rust will panic---                  \
     *  \    B. while RELEASE: rust will do wrapping....         \
     *  \       if biger then num/256 reminder= assign.          \
     *  \       256=length here.                                 \
     *  \--------------------------------------------------------\
     * */
}
