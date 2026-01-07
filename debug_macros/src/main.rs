#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}\n\n\n"); // the :? inside curly brackets tells println that we want to use an o/p format called DEBUG.

    dbg!(&rect1);
}

// dbg returns ownership of the expression's value.
