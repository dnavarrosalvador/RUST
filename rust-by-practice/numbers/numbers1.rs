// Remove something to make it work

fn main() {
    let x: i32 = 5;
    // let mut y: u32 = 5;
    // y = x; falla porque no hay conversiones de tipo
    
    let mut y = 5;
    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}

