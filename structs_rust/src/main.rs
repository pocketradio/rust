struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // let user1 = User{
    //     active : true,
    //     username : String::from("xyz"),
    //     email : String::from("xyz@gmail.com"),
    //     sign_in_count : 1,
    // }

    // dot notation to get a name from a struct. eg. user1.email , user1.active

    // mutable instance : ( whole instance must be mutable. cant mark only a particular field as mutable. )
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheruser@gmail.com");


    let user3 = User{
        email: String::from("hi@gmail.com"),
        ..user2 // struct update to get all other fields copied except for the fresh user3 email field in the struct instance for user3.
    }

    //IMPORTANT: 

    // struct update MOVES the data ; we can no longer use user2 after this because the string in the USERNAME field of user2 was moved into user3.
    // iF we had given new String values for both username and email ( the String fields ) , then since the other two fields implement the COPY trait, user2 would still be valid. 
    // we can STILL USE USER2 EMAIL here because it was not moved out of user2. only the username field was moved out. ( and other two -> copy trait so they don't matter. )

}


//field init shorthand :
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // username : username not needed. likewise email : email
        email,
        sign_in_count: 1,
    }
}



