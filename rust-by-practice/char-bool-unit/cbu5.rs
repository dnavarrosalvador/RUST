// Make it work, don't modify `implicitly_ret_unit` !
/// In Rust, the unit type is a special 
/// type that represents the absence of a value. 
/// It is written as () and is used in situations where 
/// a value is expected but not needed.

fn main() {
    let _v: () = ();
//    let v: () = ();

    let v = (2, 3);
    // assert_eq!(v, implicitly_ret_unit()); // falla porque v está asignado a un valor y la función retorna un unit()
    assert_eq!(_v, implicitly_ret_unit());  // funciona porque aqui decimos que v no se use: vacio == vacio

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
/*
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
*/