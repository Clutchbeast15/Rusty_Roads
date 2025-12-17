//Integer

//1.🌟
/*
// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}
*/
      /*//////////////////////////////////////////////////////////////
                                SOLUTION
    //////////////////////////////////////////////////////////////*/

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}

//2.🌟
/*

// Fill the blank
fn main() {
    let v: u16 = 38_u8 as _ _;

    println!("Success!");
}*/
    /*//////////////////////////////////////////////////////////////
                                SOLUTION
    //////////////////////////////////////////////////////////////*/
// Fill the blank
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");

//3.🌟🌟
 /* 
// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}
*/

    /*//////////////////////////////////////////////////////////////
                                SOLUTION
    //////////////////////////////////////////////////////////////*/
// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u16 + 8;
   let v2 = i16 ::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}
