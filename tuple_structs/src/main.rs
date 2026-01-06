// same as regular structs but they only have the TYPE, not the name associated with a field.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let color1 = Color(0, 0, 1);
    let point1 = Point(0, 0, 0);

    //destructuring : needs you to NAME the struct when you destructure them :-
    let Point(x, y, z) = origin; // destructures the values in the origin point into variables x,y and z.
}

// structs without any fields ( unit structs ):

struct AlwaysEqual;

// and in fn main or smth, -> let subject = AlwaysEqual;
