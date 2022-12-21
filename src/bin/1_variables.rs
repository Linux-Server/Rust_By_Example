fn main() {
  // variables are immutable by default
  println!("Hello Variables");
  // need to add 'mut' keyword to update a varibale
  let mut one = 1;
  println!("The values is : {:?}", one);
  // the update value should always be tha same type of orginal value --
  // for ex: string to string or integer to integer
  one = 6;
  println!("The values is : {:?}", one);

}
