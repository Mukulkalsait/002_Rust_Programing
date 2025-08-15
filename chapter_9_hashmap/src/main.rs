use std::collections::HashMap;

fn create_hash_map() -> HashMap<String, i32> {
    /* Y: 1. TYPE DEFINED */
    let mut _map_type_defined: HashMap<String, u32> = HashMap::new();

    /* Y: 1. TYPE UN-Defined. */
    let mut map_type_undefined = HashMap::new();
    map_type_undefined.insert(String::from("alice"), 15);
    map_type_undefined.insert(String::from("bob"), 12);

    map_type_undefined
}

fn access_map_values(map: HashMap<String, i32>) -> HashMap<String, i32> {
    let key = "alice";
    if let Some(score) = map.get("alice") {
        println!(
            "Finding Val With Key Pare + Error Handling.\n ∴ {} Score is {}",
            key, score
        );
    } else {
        println!("{} not found.", key)
    }
    map
}
fn remove_item_from_hash_map(mut map: HashMap<String, i32>) {
    map.remove("alice");
}

fn for_loop_in_hash_map(map: HashMap<String, i32>) {
    for (key, value) in &map {
        println!("{} : {}", key, value);
    }
    remove_item_from_hash_map(map);
}

fn print_hash_methods(mut map: HashMap<String, i32>) {
    // G: printing direct.
    println!("{:?}", map);

    // G: accessing values.
    map = access_map_values(map);
    map.insert(String::from("Mukul"), 99);

    // Y: entry() => check key | or_insert() -> if not found add.
    map.entry(String::from("Mukul")).or_insert(99);
    map.entry(String::from("Shruti")).or_insert(75);

    for_loop_in_hash_map(map);
}

fn main() {
    let map = create_hash_map();
    print_hash_methods(map);

    /* |------------------------------------------------------------------------------------------------------------
     * DX: Common Confussion in HASHMAP.                                                                            |
     * |------------------------------------------------------------------------------------------------------------
     *     1. Ownership & Borrowing: Strings & complex keys are moved unless you pass references.
     *     2. Copy vs Clone: Integers copy by default, Strings must be cloned if you want to reuse after inserting.
     *     3. Order is not guaranteed — HashMap is unordered.
     *     4. .get() returns a reference → sometimes you must dereference it with *.
     */
}
