fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 3 {
        println!("hi")
    }
    // conditions must always be a BOOL.

    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}"); // will give error since type values are
                                                  // incompatible. rust needs to known definitively at compile time what type the 'number' is.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break NEEDS a return value. semicolon needed to terminate the break statement. it returns counter * 2 back to result variable.
        }
    };

    println!("The result is {result}");
}
