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

const TWO: u32 = 1 + 1;

/* A temp function to show how constant cretaetd adn worked.
 * Y: created CONST outside FN, with CAPITAL leters. */
fn create_fetch_conts() {
    println!(
        "---------------------------------------------------\nX. TestFunction: create_fetch_conts() === > \n"
    );
    println!("Accessing Const: {TWO}"); // success
}

/* P1a: explanation of 2 Main Data Types. */
fn learn_data_types() {
    println!(
        "---------------------------------------------------\nA. Function: learn_data_types() === > \n"
    );

    /* Y: 2 main data types:
     * A. Scaller Datatypes.
     * B. Compound Datatypes.
     *
     *
     * DX:======== A Scaller ========
     *
     * G: Scaller Types.
     *  ---------------
     *  Intiger
     *  Floatpoint number
     *  Boolean
     *  Charecter
     *
     * DX:======== B COMPOUND ========
     *
     * G: B. Compound DT -> TAF
     *  ->
     *  type that represent a group of values  is "Compound data type".
     *   ----------------------------------------
     *    A. tup ( tupil)
     *    B. array
     *    C. fu
     **/

    println!(
        " 
            ===== Data Types 2(S-C)=====
     _________________________________________
    | 2 Main: Scaller and Compound Datatypes. |
    |_________________________________________|

    ======== A Scaller ========
     Types.
     ---------------
     Intiger
     Floatpoint number
     Boolean
     Charecter

    ======== B COMPOUND ========
     Type that represent a group of values  is \"Compound data type\".
     ---------------
     A. tup ( tupil)
     B. array
     C. fu "
    );

    scaller();
    compound();
}

/* FUN_1: Explanation of scaller data type INT. */
fn scaller() {
    println!(
        "---------------------------------------------------\nA. Function: learn_data_types() === > \n"
    );
    println!(" ===== SCALLER (IFBC) =====");

    /* DX:======== A Scaller ========
     *
     * G: Scaller Types. IFBC
     *  ---------------
     *  Intiger
     *  Floatpoint number
     *  Boolean
     *  Charecter
     */
    scaller_int();
    scaller_float();
    scaller_boolean();
    scaller_characters();
}

/* FUN_2: Explanation of scaller data type INT. */
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

/* FUN_2: Explanation of scaller data type Float. */
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

/* FUN_2: Explanation of scaller data type BOOLEAN. */
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

/* FUN_2: Explanation of scaller data type UNICODE CHAR. */
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

/*  FUN_1: Explanation of COMPOUND data types started. */
fn compound() {
    println!(
        "---------------------------------------------------\nA-2. Function: compound() === > \n"
    );
    println!(" ===== COMPOUND (TAF)=====");

    /* Y: ======== A Compound ========
     *  ->
     *  type that represent a group of values  is "Compound data type".
     *   ----------------------------------------
     *    A. tup ( tupil)
     *    B. array
     *    C. fu
     **/

    compound_tupil();
    compound_array();
    compound_function();
}

/* FUN_2: Explanation of COMPOUND types TUPIL. */
fn compound_tupil() {
    println!(
        "---------------------------------------------------\nA-2 .4: Function: Compound_tupil() === > \n"
    );

    /* IMP: TUPIL
     *  1. 1 + Values = tupil.
     *  2. Type can be EXPLICIT or UNSPECIFIED and DIFFERENT.
     */

    let tup1: (&'static str, i32) = ("lets get RUSTY", 100_000); // Y: we can add "_" between Numbers.
    let tup2: (&'static str, i32, &'static str) = ("string1", 1234, "i am 3rd so am i Truple?");
    let tup3: (&'static str, i32, String, bool) = (
        "string2",
        11223344,
        String::from("i am fucking ProperString Here"),
        true,
    ); // G:
    // String::from("string") = workes.

    geting_valuse_from_tupils(tup1, tup2, tup3);
}

/* FUN_3: Tupils extension. = 1 */
fn geting_valuse_from_tupils(
    tup1: (&'static str, i32),
    tup2: (&'static str, i32, &'static str),
    tup3: (&'static str, i32, String, bool),
) {
    // B: 1: De-stracturing
    let (chanel_name, subcriber_count) = tup1;
    let (a, b, c) = tup2; // this totally worked. + automatic assign.

    // B: 2: Dot-Notatioon
    let subcount1 = tup3.0;
    let subcount2 = tup3.1;
    let subcount3 = tup3.2;

    /*B: geting values from -> TUPIL
     *   --------------------------------
     *  1: tupil De-stracturing
     *    we are going to take all the valuse
     *    in a single go with variable for every
     *    value in tupil
     *  ---------------------------------
     *  2: tupil Dot-Notatioon.
     *    we will only take the value we
     *    needed and use it.
     * */
}

/* FUN_2: Explanation of COMPOUND types ARRAY. */
fn compound_array() {
    /* IMP: Array:
     *===============================
     *  1. fixed Length (otherWise use VECTOR)
     *  let arrName = [ a , 'b', 23 , javan]  ==> array.
     *  let arrayNm = [0;8] in this line we are making array of "8"
     *  value whereas all 8 numbers are  "0";
     * */

    let error_code_array = [200, 404, 500];
    let nof_found = error_code_array[1]; // = 404
    let array_of_8_zeros = [0, 8];
}

/* FUN_2: Explanation of COMPOUND types FUNCTIONS. */
fn compound_function() {
    function_creation_rules_n_info();
}

/* FUN_3: Explanation of function types. = 1 */
fn function_creation_rules_n_info() {
    /* B: FN
     *   1. Can have arguments -> multyple just like normal funciotns in any languages
     *   2. Same num of arguments must be passed while calling.
     */
    /* G: any code in RUST is --> STATEMENT or EXPRESSTION.
     *   a. statement => perform some action but do not return a value.
     *   b. expression => perform some action + return a value.
     *   EG.
     *
     */

    println!("a statement. "); // Y: is a statement
    let sum = returning_function_one(23, 34); // Y: EXPRESSTION
    let multiply = returning_function_two(23, 34); // Y: EXPRESSTION
    //
    println!(
        "
The Sum of returning_function_one(x:i32,y:i23)->i32 is = {}
The multiply of returning_function_two(x:i32,y:i23)->i32 is = {}
",
        sum, multiply
    ); // Y: is a statement
}

/* FUN_3: Explanation of function types. = 2 */
fn returning_function_one(x: i32, y: i32) -> i32 {
    /* IMP: the proper Way is
     * ||----------------------------||
     * || fn function_name() -> i32{ ||
     * || return sum                 ||
     * || }                          ||
     * ||----------------------------||
     *
     * Y:
     * 1. The function will give ERROR if [ -> i32 ] return type is not specified.
     * 2. No need of ';' semicolon or "return" in last sentence. ( but you can still use them.)
     */

    let sum = x * y;
    sum
}

/* FUN_3: Explanation of function types. = 3 */
fn returning_function_two(x: i32, y: i32) -> i32 {
    x * y
}
