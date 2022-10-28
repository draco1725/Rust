//when you are making a rust program always use "main" it wont work

fn main() {
  run();
}

fn run() {
  println!("Hello World");
}


//Lets try with another here we making another program but here we passmout values too

fn main() {
  run(2,3);
}

fn run(x:i32,y:i32) { //here we are define what type of values are those by using integar and unsigned integar. 
  println!("{}","{}",x,y);
}


//now if we return the value lets see how we can do it

fn main() {
  println!("{}",run(2,3)); //here also we need to print the value
}

fn run(x:i32,y:i32) -> i32{  //now here have returned the value by stating it  
  //println!("{}","{}",x,y);
  return x*y
}
