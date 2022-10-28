//if else

fn main(){
  let x = 10;

  if x == 11{
      println!("x is 10");
  }

  else {
      println!("x is 11")
  }
}



//loops

fn main () {
  let mut count = 0;
  loop {
      count = count + 1;
      println!("Hello world");
      if count == 10{
          break;
        }
  }

}
