fn main() {


    println!("Hello, world!");
}



│Array = fixed-size | same type | stack memory | useful when known at compile time
   1   │
   2   declaration
   3   │ let arr: [i32; 5] "✅5 elements, each of type i32" = [1, 2, 3, 4, 5];
   4   │ let arr = [0; 5]; // [0, 0, 0, 0, 0]
    5   │ let a = [1, 2, 3];              // inferred type
       6   │ let b: [i32; 3] = [4, 5, 6];    // explicit type
          7   access
             8   │ let x = a[0];   // 1
                9   │ let y = b[2];   // 6
                  10   │ println!("{}", a[5]); // PANIC: index out of bounds
                    11   │ println!("{:?}", a);     // Debug print whole array.
                      12
                        13   methods-
                          14    arr.len()
                            15    arr.is_empty()
                              16    arr.iter()
                                17    arr.contains(&value)
                                  18    arr.copy_from_slice(&other_arr)
                                    19    arr.clone() (if element type supports Clone)
                                      20   slice =
                                        21   │     let a = [1, 2, 3, 4, 5];
                                          22   │     let slice = &a[1..4]; // [2, 3, 4]
                                            23   │ split_at(idx)
                                              24   │ chunks(n) / windows(n)
                                                25   │ first(), last()
                                                  26   │ get(i) (safe access, returns Option)
                                                    27   │ starts_with(&[T]), ends_with(&[T])
                                                      28   Vectors:
                                                        29   .resize(new_len, value)
                                                          30   .truncate(n) – remove elements
                                                               after n
                                                                 31   .dedup() – remove
                                                                      consecutive duplicates
                                                                        32   .capacity() – check
                                                                             current capacity
