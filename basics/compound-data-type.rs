//Array
fn main() {
  let mut a = [1,2,3,4];
  arr[3] = 5;  //here we changing the values in array and here use "mut" 
  println!("{}",a[3]);
}


//tupple
fn main() {
  let tup = (10,3.14,'a',true);
  println!("{}",tup.2);
}

// Tuples
fn tuples() {
    let tup: (i32, i32, String) = (1, 2, String::from("Paul"));
    println! ("{}", tup.0);
    println! ("{}", tup.1);
    println! ("{}", tup.2);
    let mut tup: (i32, 132, String) = (1, 2, String::from("Paul"));
    tup.2 = String::from("John");
    println! ("{}", tup.0);
    println!("{}", tup.1);
    println! ("{}", tup.2);
