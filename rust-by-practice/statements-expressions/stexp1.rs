// Make it work with two ways
fn main() {
   let v = {
       // let mut x = 1;
       // En una expresion no hace falta declarar x como mutable
       // x = 1 es un valor de inicializacion para la expresion
       let x = 1;
       // x += 2
       x + 2 // esto si funciona
   };

   assert_eq!(v, 3);

   println!("Success!");
}