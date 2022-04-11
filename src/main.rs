fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // s's value moves into the function...
                                   // ...and so is no longer valid here
    let x = 5;                     // x comes in to scope

    makes_copy(x);                 // x moves into the function, but
                                   // but i32 is Copy, so it's ok to still
                                   // use x afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and "drop" is called, the
  // underlying memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope, since there was no heap
  // allocation, nothing is freed either
