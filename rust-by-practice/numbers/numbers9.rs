// rangos
// https://doc.rust-lang.org/std/ops/struct.Range.html
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        // solo se incluye desde -3 hasta -1, solo 5 elementos
        println!("{}", i);
        sum += i;
        println!("{}", sum);
    }

    assert!(sum == -5);

    // el = en un rango es para incluir el ultimo elemento, por defecto no se incluye
    for c in 'a'..='z' {
        println!("{}",c); // esto imprime caracteres de a a z
        println!("{}",c as u8); // esto imprime codigos ASCII de 97 a 122
        // el keyword 'as' hace un cast de tipos
        // https://doc.rust-lang.org/std/keyword.as.html

    }
}
