
fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me();
}

/*//////////////////////////////////////////////////////////////
                                SOLUTION
  //////////////////////////////////////////////////////////////*/

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(5); // Corrected to pass a u8 value
}
