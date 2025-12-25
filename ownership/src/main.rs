fn main() {
    let s = String::from("hello");

    take_ownership(s); // s MOVES into the take ownership's scope. the first value is invalidated in the stack.

    // using s anymore -> error. because it has moved into the take ownnership function now

    let x = 5;
    make_copy(x); // x does nOT move into the function because its an integer ( fixed size ) and its stored directly on the stack.
                  // the function instead gets a COPY.

    let s2 = String::from("hiiii");

    let s3 = take_and_give_back(s2) // so s2 is invalidated here since it moves into the function. then the function returns a value which 
    // moves into s3. 

} // x goes out of scope here. s3 goes out of scope. s2 was moved, so nothing happens. 

fn take_ownership(s2: String) {
    println!("the string is {s2}");
} // s2 moves out of scope and DROP is called -> memory is freed in the heap and the stack.

fn make_copy(int1: i32) {
    println!("integer is {int1}");
} // int1 goes out of scope. removed from stack.


fn take_and_give_back(str1 : String){

    str1 // returns back to fn call
}
