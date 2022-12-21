// Stattement Vs Expressions
// Statement ends with a semicolen for ex: let x=10;
//But expression doesnt have semicolen and expression definitely returns something
/*for ex: fn examp()->i32{
5+10
}*/


fn main(){
  let another = another_func(128);
  println!("Another func result : {:?}", another);
}

fn another_func(x: i32)-> String{
  println!("Another function called is : {:?}", x);
  "hello".to_string()
}