fn main() {
    
    let s = String::from("hello world");

    // String slicing syntax
    let hello = &s[0..5]; // A new reference for part of s
    let world = &s[6..11];

    // All of these declarations work
    let word = new_first_word(&s[0..6]);
    let word = new_first_word(&s[..]);
    let word = new_first_word(&s);

    let my_string_literal = "hello world";

    let word = new_first_word(&my_string_literal[0..6]);
    let word = new_first_word(&my_string_literal[..]);

    // Works because string literals are slices already
    let word = new_first_word(my_string_literal);

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Converts the string s into an array of bytes

    // Iterates over the array of bytes
    // Iter is a method that returns each element in a collection
    // Enumerate wraps the result of iter and returns each element as part of a tuple
    // i(the index and the first element returned from enumerate)
    // &item(the second element return from enumerate and a reference to the element)(also a single byte in the tuple)
    for (i, &item) in bytes.iter().enumerate() {
        // Using byte syntax to find a space character
        // If space char is found, index is returned
        if item == b' ' {
            return i;
        }
    }
    
    // Return the length of the string if no space is found
    s.len()
}

// Improved first word function
fn new_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
