// Make it work
fn main() {
    // let c1 = "中"; // error, tiene doble comillas, se trata como un string
    let c1: char = '中'; // esto si funciona correctamente
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}