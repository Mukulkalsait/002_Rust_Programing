/* Takes out even odd.*/
pub fn even_or_odd(num: i32) -> (i32, String) {
    let res = if num % 2 == 0 {
        String::from("Even")
    } else {
        String::from("Odd")
    };
    (num, res)
}

/* Takes out even odd but use reference in it.*/
pub fn even_or_odd_with_reference<'a>(num: &'a i32) -> (&'a i32, String) {
    let res = if *num % 2 == 0 {
        String::from("Even")
    } else {
        String::from("Odd")
    };
    (num, res)
}

/* Difference between &str and String like names.*/
pub fn difference_between_same_name_types() {
    fn string_vs_str() {
        /* Y:
         * |-----------------------------------------------------------------------------------------
         * |                                    &String vs &str                                     |
         * |-----------------------------------------------------------------------------------------
         * |  #String =>    1. heap-allocated
         * |                 2.growable string type.
         * |  #str    =>    1. string slice
         * |                2. a view into some UTF-8 data
         * |                3. immutable sequence of characters in memory.
         * |&String points -> String struct (containing pointer to the heap buffer, length, capacity).
         * |&str points directly to actual char in mem (no length/capacity metadata from String).
         * |In Rust
         * |      => &String coerce automatic into &str
         * |          cos mostly you just want read data.
         * |-----------------------------------------------------------------------------------------
         * B: Usage:
         *   ∴ If you don't need specific String methods => take a &str parameter for flexibility (it works for both String and string literals).
         *   and => Use &String only if you specifically require the String type.
         */

        let s: String = String::from("hello");
        let string_ref: &String = &s; // &String — a reference to the whole String object
        let str_ref: &str = &s; // &str — a reference to the string's data (string slice)
        println!(
            "
            string_ref: {} -> a reference to the whole String object
            str_ref: {} -> a reference to the string's data (string slice)
            ",
            string_ref, str_ref
        );
    }
    string_vs_str();

    fn box_vs_and_t() {
        let b: Box<i32> = Box::new(42);
        let ref_box: &Box<i32> = &b; // Reference to the box
        let ref_val: &i32 = &b; // Reference directly to the inner value (deref coercion)
        println!(
            "
            Reference to the box                                       ref_box : {}
            Reference Directly to the innner value (dref co-er-cion)   ref_val : {}
            ",
            ref_box, ref_val
        );

        /* Y:
         * &Box<T> vs &T
         *     Box<T>    -> 1. heap-allocated smart pointer
         *                   2. owns its data.
         *     &Box<T>   -> 1. reference to the box obj
         *                   2.  itself is a pointer to T.
         *     &T is a reference directly to the value of type T.
         *
         * G: facts
         * You can go from &Box<T> → &T automatically via deref coercion.
         * In practice, most of the time you don't store &Box<T> — you either store Box<T> or &T.
         */

        use std::ops::Deref; //addvance Deref 
        #[derive(Debug)]
        struct MyBox<T>(T);

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let m = MyBox(String::from("hello"));
        let ref_box: &MyBox<String> = &m;
        let ref_str: &String = &m; // Works because of Deref
        println!(
            "
            ref_box: &MyBox<String> = &m;                       => {:?}
            ref_str: &String = &m; // Works because of Deref    => {:?}
            ",
            ref_box, ref_str
        );
    }
    box_vs_and_t();
}
