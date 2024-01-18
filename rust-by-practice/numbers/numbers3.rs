// Modify `assert_eq!` to make it work
fn main() {
    let x = 5;
    // assert_eq!("u32".to_string(), type_of(&x));
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// el argumento de la función tiene '_' porque no vamos a usar el contenido de la variable, da igual lo que tenga
// lo que nos interesa es el tipo de la variable y usamos una referencia al mismo con &T
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>()) // devuele "i32" en tipo string
}
