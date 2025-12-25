fn main() {
    let s1 = String::from("hello");
    let len = calculate(&s1);
    println!("the length of string {s1} is {len}");

    let s = String::from("hi");
    // change(&s);
    change_mut(&mut s); // mutable reference unlike line 7 -> so the change_mut fn will mutate the value of s.
}

fn calculate(s: &String) -> usize {
    // s does NOT have ownership. it is BORROWING.
    s.len() // gets returned back to main
}

fn change(str1: &String) {
    // str1.push_str("there");
    // above line : throws error. because we're modifying a borrowed object.
}

fn change_mut(str1: &String) {
    str1.push_str("world");
}

// Note : you can have only 1 mutable reference to a value. eg. doing r1 = &mut s; and r2 = &mut s -> error.
