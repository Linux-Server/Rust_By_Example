fn main(){
  //IF Statement
  let condition = true;
  // normal case
  if condition{
    println!("Its a true conditiom");
  }
  else{
    println!("Its a false condition");
  }

  // if condition with let

  let outcome = if condition{ 5 } else { 10};
  println!("The outcome is {:?}", outcome);
  
  // loop condition

  loop{
    println!("Sachin calling....");
    break;
  }

  // returning values from loop

  
}