fn main() {
    /* Datatypes are mainly two tyes
     Scalar datattypes and compound data types
     ~ Scalara data type can hold only single value, for ex: intger, string,
    floatingpoint, char etc*/
    let _alice = "Alice"; // string type
    let _one = 1; //integer type
    let _a = 'A'; // char type

  //TUPLE TYPE ---  
  
    // compound data types can group multiple other types like Struct, tuple
    // tuple will group data of different data type
  // tuples are always fixed size
    let _tuple = (1, 2, 3, 4, 5);
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // Inorder  to get invidual data from a tuple : we need destructuring
    let (x, y, z) = _tup;
    println!("The first element: {:?}", x);

    // we can also access each element by using a period (.)
    println!("The third element is : {:?}", _tup.2);

  //ARRAY TYPE ---

    //array is also fixed size
  // it cannot grow or shrink
   // unlike tuples, array can only accomadate data of same type
  let array_one = [1,2,3,4,5];
  let array_two = ["alice", "bob"];
  let a: [i32; 5] = [1, 2, 3, 4, 5]; // the  type annotation of array will look like this 
  let a = [3; 5]; // we can declare big array like this ex: [3,3,3,3,3]

  println!("The array is {:?}", a[1]); // to access individual element of an array will be like this

  

  //both array and tuple stypes are stored in stack memory 
  
}
