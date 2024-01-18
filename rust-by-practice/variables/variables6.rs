// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    //let x = x; // Prints "7".
    // aqui x esta redeclarada y ahora es inmutable
    // toda variable es inmutable por defecto
    // x += 3; estp falla porque hay que declarar x como mutable
    // corrijo a continuacion comentando la linea 6
    x += 3;
    println!("{}", x); // Prints "10".


    // let y = 4;
    let y: i32 = 4;
    // Shadowing
    let y: &str = "I can also be bound to text!"; 
    // el compilador saca un warning de variable no usada

    println!("Success!");
}