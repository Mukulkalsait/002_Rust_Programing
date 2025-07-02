fn main() {
    println!(
        "---------------------------------------------------\n MAIN() :Start\n---------------------------------------------------"
    );
    println!("const can be access BEFORE DECLERATION:{TWO}"); // Y: const can be access BEFORE DECLERATION.

    create_fetch_conts();
    learn_data_types();
    scaller_float();
    scaller_boolean();
    scaller_characters();

    println!(
        "---------------------------------------------------\n MAIN() === END\n---------------------------------------------------"
    );
}

const TWO: u32 = 1 + 1;
/* A temp function to show how constant cretaetd adn worked.
 * Y: created CONST outside FN, with CAPITAL leters. */
fn create_fetch_conts() {
    println!(
        "---------------------------------------------------\nX. TestFunction: create_fetch_conts() === > \n"
    );
    println!("Accessing Const: {TWO}"); // success
}

/* explanation of datatypes. */
fn learn_data_types() {
    println!(
        "---------------------------------------------------\nA. Function: learn_data_types() === > \n"
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

/* explanation of scaller data type INT. */
fn scaller_int() {
    println!(
        "---------------------------------------------------\nA.1: Function: scaller_int() === > \n"
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
        "a:{}\nb:{}\nc:{}\n=>(
        The valuse of d and e are not
printed cos we used _d and _e to tell compailer
that we are not using it and we are doing it on
purpose.
) \nf:{}",
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

/* explanation of scaller data type Float. */
fn scaller_float() {
    println!(
        "---------------------------------------------------\nA.2: Function: scaller_float() === > \n"
    );
    // IMP:  2. FLOAT:
    let g = 2.0; //  G: float default f64 IMP"f32= float32"
    let h: f32 = 3.0; //  forcefully assgined f32
    // let higher_flot: f128 = 32934945.22349234; // Y:so higher float is avialable but we havent run it.

    println!(
        "g:{}\nh:{}\nhigher_flot: right now no supporting hardwear is avialable.",
        g, h
    );
}

/* explanation of scaller data type BOOLEAN. */
fn scaller_boolean() {
    println!(
        "---------------------------------------------------\nA.3: Function: scaller_boolean() === > \n"
    );
    // IMP:  3. Boolean:
    // Can be unset or Forcefully-set.

    let _i = true; // un-set
    let _j: bool = false; // B:set we forcerully set a bull here
    // Y: bool allways takes up 1 BYTE ( 8 bit ) Space.
    //  and we are INTENTIONALLY not USING i and j so
    //  i = _i and j = _j. to stop warning.
}

/* explanation of scaller data type UNICODE CHAR. */
fn scaller_characters() {
    println!(
        "---------------------------------------------------\nA.4: Function: scaller_characters() === > \n"
    );
    /* IMP:  4. Characters
     * ===========================
     * \        Characters:       \
     * \--------------------------\
     * \    unicode characters    \
     * \ allways in SINGLE COTE ''\
     * \ ' ' is allways = UNICODE \
     * \==========================\
     */
    println!(
        "a length of CHAR in RUST is allways = {} Bytes, or 32bit.",
        std::mem::size_of::<char>()
    );

    let k = 'z';
    let l = 'Z'; //captial "Z"
    let heart_eyed_cat = '😻'; // i dont  know how did this became unicode ? 
    println!(
        "Deu to the Unicode Nature of characters. 
The 4 bytes can contian anything that comes under unincodes.
including emojis. 
see k: {}
l: {}
heart_eyed_cat: {}
",
        k, l, heart_eyed_cat
    )
}
