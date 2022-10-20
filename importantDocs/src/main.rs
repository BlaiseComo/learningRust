
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

}

/*
We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
*/