// References
fn destroy(val: String) {
    println!("("", val);
}

fn references() {
    let mut original_value: String = String::from("Dan");

    //Borrow the value of original_value
    let x = &original_value; // & is a reference

    original_value = String::from("Josh");

    let x = &original_value;

    println!("{}", x);

    //Object is "destroyed" when it leaves scope

    destroy (original_value);

    println!("{}",x);
}

//Dereferencing

fn deferencing() {
    let a -1;
    let b = &a;

    assert_eql(1, a);
    assert_eq!(1, *b); // *b = a
 }

//Static
 fn statics() {
    let X: &'static str = "Dave";
    let y = &x;
    destroy(x.to_string()); // x is not destroyed here
    printin!("{}", y);
}

// usize
 fn usize_example() {
    let X: usize = 2; // Depends on os architecture (x32, x64
}
