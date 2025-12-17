//Destructuring
//🌟🌟 We can use a pattern with let to destructure a tuple to separate variables.

/*

// Fix the error below with least amount of modification
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
*/

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/

fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}



// Destructuring assignments
// Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

// 🌟🌟
/*


fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
} 
*/

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
 //////////////////////////////////////////////////////////////*/

fn main() {
    let (mut x, mut y);
    (x,y) = (3, 4);
    [x, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [1,2]);

    println!("Success!");
} 
