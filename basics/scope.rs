//it shows how much reachability is there

fn main() {
  let x=10;

{
    let y =100;
    println!("{} {}",x,y);
}

  {
    let z =200;
    println!("{} {}",x,z);  //here "Y" value wont come because its not inside his scope 
  }

println!("{}",x);
}
