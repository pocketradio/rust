fn main() {
    let s = String::from("hello");

    take_ownership(s); // s MOVES into the take ownership's scope. the first value is invalidated in the stack.

    // using s anymore -> error. because it has moved into the take ownnership function now

    let x = 5;
    make_copy(x); // x does nOT move into the function because its an integer ( fixed size ) and its stored directly on the stack.
    // the function instead gets a COPY.

    let s2 = String::from("hiiii");

    let s3 = take_and_give_back(s2); // so s2 is invalidated here since it moves into the function. then the function returns a value which 
    // moves into s3.
} // x goes out of scope here. s3 goes out of scope. s2 was moved, so nothing happens. 

fn take_ownership(s2: String) {
    println!("the string is {s2}");
} // s2 moves out of scope and DROP is called -> memory is freed in the heap and the stack.

fn make_copy(int1: i32) {
    println!("integer is {int1}");
} // int1 goes out of scope. removed from stack.

fn take_and_give_back(str1: String) -> String {
    str1 // returns back to fn call
}

/*

String::from → owner A
move → owner B
move → owner C
scope ends → DROP (once)



If str1 is not returned

fn take_and_drop(str1: String) {
    println!("{str1}");
}

What happens:

1. Ownership of the String moves into str1


2. Function ends


3. str1 goes out of scope


4. Rust calls Drop::drop



What drop actually does

Heap: frees the allocated string buffer

Stack: the String struct itself (pointer, len, capacity) is popped


So yes:

> Heap memory is freed, and the stack slot is removed.




---

Why returning prevents drop

fn take_and_give_back(str1: String) -> String {
    str1
}

str1 is moved at the return expression

A moved value is not dropped

Ownership transfers to the caller

Caller’s variable becomes responsible for dropping later



---

Important mental model

drop runs once

It runs for the final owner

Moves just change who the owner is



---

Ownership lifecycle (visual)

String::from → owner A
move → owner B
move → owner C
scope ends → DROP (once)


---

One-line rule

> If a value goes out of scope without being moved, it is dropped.

*/
