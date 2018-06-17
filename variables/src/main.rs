// ### immutable variables
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// ### mutable variables
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// ### Shadowing
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }

// ### we can use the same name of a variable with shwaowing,
// ### even change type of the variable
// fn main() {
//     let spaces = "     ";
//     let spaces = spaces.len();
//     println!("The value of spaces is: {}", spaces);
// }

// ### but if we use "mut", that will throw an error
// fn main() {
//     let mut spaces = "     ";
//     spaces = spaces.len();
//     println!("The value of spaces is: {}", spaces);
// }
