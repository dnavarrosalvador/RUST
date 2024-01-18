// SCOPE VARIABLES
// Fix the error with the use of define_x
fn main() {
    // println!("{}, world", x); 
    // no funciona porque x no esta definido
    // lo corrijo haciendo define_x
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x); 
}
