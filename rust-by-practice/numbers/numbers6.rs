// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    // esto es curioso: 1_024 es el numero 1024 en decimal, el '_' es un separador
    // 0xff es el numero 255 en decimal, el '_' es un separador
    // 0o77 es el numero 77 en octal, el '_' es un separador
    // 0b1111_1111 es el numero 31 en binario, el '_' es un separador
    //  assert!(v == 1579); // se produce un panic en el thread 
    assert!(v == 1597);

    println!("Success!");
}
