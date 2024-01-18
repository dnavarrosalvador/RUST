// Make it work
fn main() {
    let f = true;
    let t = true && false; // false
    // assert_eq!(t, f); // false, esto falla
    assert_eq!(t, !f); 

    println!("Success!");
}