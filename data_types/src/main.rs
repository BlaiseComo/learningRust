fn main() {

    // VECTOR BEGIN
    
    // Declaration of a vector that will hold i32 integer(s)
    let mut v: Vec<i32> = Vec::new();

    // Declaration of a new vector initialized with i32 integers
    let v2 = vec![1, 2, 3];

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    // The get method returns an Option type which you can use with match
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Adding values to the back of vector v
    v.push(5);
    v.push(6);
    v.push(7);

    // Looping through a vector
    for i in &mut v {
        // Adds 50 to each element
        *i += 50;
    }

    // Creating a vector of different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // VECTOR END

    // STRINGS BEGIN

    // Creates an empty string called s that can have data loaded into it
    let mut s = String::new();

    let data = "initial contents";

    // Converts the string literal data into a String
    let s = data.to_string();

    // The above statement can also be done via
    let s = String::from("initial contents");

    // Next two lines create a string called foobar
    let mut s = String::from("foo");
    // Push_str wouldn't take ownership of "bar" even if it was a seperate variable
    s.push_str("bar");

    // To add two strings using the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used

    // Adds three strings together using !format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // STRINGS END

    // HASH MAPS BEGIN

    

}
