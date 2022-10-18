fn main() {

    let mut x = 5; // if you remove the mut from this code, it will result in a compilation error

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}"); 

    let y: f64 = 4.16;

    println!("The value of y is: {y}");

    let tup = (500, 6.4, 1);   
    let (x2, y2, z) = tup;

    println!("The value of y2 is: {y2}");

    let tup_x2 = tup.0;

    println!("The value of x2 is: {tup_x2}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // This array will contain 5 elements all set to the value of 3

    // Values of an array can be accessed with indices similar to c++

}
