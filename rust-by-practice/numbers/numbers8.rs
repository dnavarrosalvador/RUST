// meke it work in two different ways
fn main() {
    // assert!(0.1+0.2==0.3); // se produce un panic en el thread
    assert!(0.1_f32+0.2_f32==0.3_f32); //arreglado porque hay que especificar el tipo de dato!!!

    println!("Success!");
}
