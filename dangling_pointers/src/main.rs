fn main() {
    // let ref1 = dangle();
    let ref1 = no_dangle();
    println!("{ref1} is the ref1");
}

// fn dangle() -> &String {
//     let s = String::from("hi");

//     &s
// }

// issue above : s goes out of scope ( dealloc ) after this. so there is nothing in memory. but &s is being returned and stored in ref1.
// hence the reference will be pointing to an invalid String.

fn no_dangle() -> String {
    let s = String::from("hi");

    s
} // s is not deallocated here ; its MOVED into ref1 in main. works fine.
