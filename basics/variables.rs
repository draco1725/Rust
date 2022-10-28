//Lets type a variable here

fn main() {
  let num = 10;  //num is immutable here
//to make it mutable we use "mut" for eg: let mut num =10;
//instead of "let" we can also use "const" also and its faster also
  num = 5;
  println!("{}",num);
}


//to use const we type the below code
//Syntax: const NEW_NUM:F64=3.14;

fn main() {
  const PIE:f64 = 3.14;
  println!("{}",PIE * 10.0 * 10.0);
}
