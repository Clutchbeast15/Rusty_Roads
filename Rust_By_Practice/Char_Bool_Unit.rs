Char
1.🌟

// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
} 

  /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/


// Make it work
use std::mem::size_of_val;
fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 


2.🌟

// Make it work
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/


// Make it work
fn main() {
    let c1: char = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}


Bool
3.🌟


// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    }
} 

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
//////////////////////////////////////////////////////////////*/


// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
} 


4.🌟

// Make it work
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/


// Make it work
fn main() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}


Unit type
5. 🌟🌟

// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/


 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/


 /*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/
