// Fix errors and panics to make it work
fn main() {
    // interesante forma de asignar tipos de variables
   // let v1 = 251_u8 + 8;   // error porque el compilador hace overflow al forzar el tipo de 8 a u8
   let v1 = 251_u16 + 8;   
   // let v2 = i8::checked_add(251, 8).unwrap(); // con i8 no se llega a representar 251
   // let v2 = u8::checked_add(251, 8).unwrap(); // con u8 el compilador no se queja pero se produce overflow
   let v2 = i16::checked_add(251, 8).unwrap(); // aqui no hay error
   // checked_add devuelve resultado de la suma si no overflow o None si overflow
   println!("{},{}",v1,v2);
}
