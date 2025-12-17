//Scope
//A scope is the range within the program for which the item is valid.

/*🌟
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}
*/

   /*//////////////////////////////////////////////////////////////
                                SOLUTION
    //////////////////////////////////////////////////////////////*/

// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
     let y: i32 = 5;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}



/*🌟🌟
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}*/

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
 //////////////////////////////////////////////////////////////*/
// Fix the error with the use of define_x
fn main() {
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x); 
}
