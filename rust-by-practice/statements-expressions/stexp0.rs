//  Examples
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
        // IMPORTANTE: FIJARSE EN EL PUNTO Y COMA AL FINAL DE LA LINEA
        // ESO DESTRUYE LA EXPRESION y z acaba teniendo ()
        // SI NO TUVIERA PUNTO Y COMA AL FINAL DE LA LINEA ENTONCES asigna 2*x a z
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}