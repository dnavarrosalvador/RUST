// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    // aqui nunca entra en la condicional
    if !t {
        println!("Success!");
    }

    // aqui si entra en la condicional
    if t {
        println!("Success!");
    }
} 