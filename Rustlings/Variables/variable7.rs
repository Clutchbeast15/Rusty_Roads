//Binding and mutability


/* 
1.Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/
  /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/


// Fix the error below with least amount of modification to the code
fn main() {
    let mut x: i32 = 5; // Uninitialized but used, ERROR !
    let mut y: i32 = 10; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

/*
2.🌟 Use mut to mark a variable as mutable.

// Fill the blanks in the code to make it compile
fn main() {
    let __ __ = 1;
    __ += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}
*/

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/

// Fill the blanks in the code to make it compile
fn main() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}



