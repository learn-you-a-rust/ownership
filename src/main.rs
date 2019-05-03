fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // s's value moves into the function
                                   // ... so it's no longer valid here

    let x = 5;                     // x comes into scope
    
    makes_copy(x);                 // x would move into the function,
                                   // but i32 is Copy, so it's okay to 
                                   // still use x afterward

    //let _t = s;                  // this doesn't work
    let _y = x;                    // but this does work

} // s and x would go out of scope, but s is already out of scope and x
  // is on the stack, so it doesn't matter.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called, freeing the memory

fn makes_copy(some_int: i32) { // some_int comes into scope
    println!("{}", some_int);
} // some_int goes out of scope, but since it was on the stack, no memory
  // needs to be freed
