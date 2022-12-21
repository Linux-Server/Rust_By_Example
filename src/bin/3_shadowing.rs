fn main(){
  //we can declare a new variable with same name of already declared variable is called - SHADOWING
  let sam = 190;
  println!("The first varibale is {:?}", sam);
  let sam = "Killer";

  //The shadowing allow us to change the type of varibale, which is main difference between shadowing a mut

// The shadowing should use the let keyword again
  //The second variaable over shadows the first one
  println!("The second varibale is {:?}", sam);

  
}