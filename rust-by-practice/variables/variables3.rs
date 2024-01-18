
// SCOPE VARIABLES
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // ERROR porque y esta declarada dentro - solo vale dentro del bloque
    // println!("The value of x is {} and value of y is {}", x, y); 
}
