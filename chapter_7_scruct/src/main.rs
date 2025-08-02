fn main() {
    struct_learning();
    // advanced_standard_fmt_1();
}

fn struct_learning() {
    // An attribute to hide warnings for unused code.
    #![allow(dead_code)]

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // A unit struct
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    fn main() {
        // Create struct with field init shorthand
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };

        // Print debug struct
        println!("{:?}", peter);

        // Instantiate a `Point`
        let point: Point = Point { x: 5.2, y: 0.4 };
        let another_point: Point = Point { x: 10.3, y: 0.2 };

        // Access the fields of the point
        println!("point coordinates: ({}, {})", point.x, point.y);

        // Make a new point by using struct update syntax to use the fields of our
        // other one
        let bottom_right = Point {
            x: 10.3,
            ..another_point
        };

        // `bottom_right.y` will be the same as `another_point.y` because we used that field
        // from `another_point`
        println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

        // Destructure the point using a `let` binding
        let Point {
            x: left_edge,
            y: top_edge,
        } = point;

        let _rectangle = Rectangle {
            // struct instantiation is an expression too
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right: bottom_right,
        };

        // Instantiate a unit struct
        let _unit = Unit;

        // Instantiate a tuple struct
        let pair = Pair(1, 0.1);

        // Access the fields of a tuple struct
        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        // Destructure a tuple struct
        let Pair(integer, decimal) = pair;

        println!("pair contains {:?} and {:?}", integer, decimal);
    }
}

fn advanced_standard_fmt_1() {
    // This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`.
    #[allow(dead_code)]
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation required to make this `struct` printable with `fmt::Debug`.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
