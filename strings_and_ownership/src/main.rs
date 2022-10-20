
fn main() {

    // This type of string is different from a string literal as it is allocated on the heap and can store unkown sizes
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string, this cannot be done with a string literal

    println!("{}", s); // This will print 'hello, world!'

    // After s2 gets assigned the pointer value of s, s is no longer valid
    let s2 = s; 
    // If you did want to copy all of the heap data from s, 
    // you could say let s2 = s.clone();

    println!("{}", s2); 

    // BREAK

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);               

    // BREAK

    

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.