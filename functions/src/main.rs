fn main() {
    println!("Hello, world!");
    another_function(85);

    let y = {
        let x = 3;
        x + 1 // EXPRESSION, NOT A STATEMENT. AN EXPR RETURNS SOMETHING. PUTTING SEMICOLON ( STATEMENT ) -> NO RETURN. 
    };
    println!("y is {y}");
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("the value of x is {x}");
}
