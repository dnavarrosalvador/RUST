// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    // println!("Size of char is {}", size_of_val(&c1));
    // TODOS LOS CARACTERES SON 4 BYTES
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    // println!("Size of char is {}", size_of_val(&c2));
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 