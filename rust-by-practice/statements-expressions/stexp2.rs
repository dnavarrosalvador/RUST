fn main() {
   // let v = (let x = 3); // aqui v es asignado el valor de la expresion logica y el assert falla
   let v = {let x = 3; x}; // como es una expresion hay que devolver el valor de la expresion

   assert!(v == 3);

   println!("Success!");
}