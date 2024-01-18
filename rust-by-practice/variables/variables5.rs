// SHADOWING 
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        // assert_eq!(x, 5);
        // aqui falla porque assert evalua x = 5, ha pisado x = 5
        // corrijo a continuacion
        assert_eq!(x, 12);
    }

    // assert_eq!(x, 12);
    // aqui falla porque assert evalua x = 5, ha pisado x = 12
    // corrijo a continuacion
    assert_eq!(x, 5);


    let x = 42;
    println!("{}", x); // Prints "42".
}
