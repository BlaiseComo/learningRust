fn main() {
    
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was exact");
    } else {
        println!("condition was false");
    }

    // BREAK

    let condition = true;
    // If you use conditionals like this, the values must be of the same type
    // So you could not have if condition {5} else {"six"};
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // BREAK

    let mut count = 0;
    // loop labels must begin with a single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // Rust can break out of outer loops
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // BREAK 

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // BREAK

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // BREAK

    // .rev() reverses the range
    // writing this as for number in (4..1) {} will not run the body of the code
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
