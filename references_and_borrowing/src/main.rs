fn main() {
    
    let s1 = String::from("hello");

    // This is how you pass a string to a function by referece rather than move it to that function
    // The function must also be changed for this to work as seen below
    // You can use * to dereference
    let len = calculate_length(&s1);

    println!("The length of {s1} is {len}");

    let mut string1 = String::from("hello"); // This string can be changed by functions
    // String1 cannot be set as the value of other variables more than once at a time

    change(&mut string1);

}

fn calculate_length(s: &String) -> usize { // This function cannot change s
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
