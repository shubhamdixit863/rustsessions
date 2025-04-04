use std::fmt::{Debug, Display, Formatter};
use std::thread;



// struct Data{
//     
// }
// 
// impl Clone for Data {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }
// 
// impl Copy for Data{
//     
// }

// mutability ,ownership and drop semantics ,lifetimes

// trait Debug

// trait Display


// struct Customtype{
//     
// }
// 
// impl Debug for  Customtype{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

fn main() {

    // by default any variable that you create in rust is immutable

    // let mut c:i32=3;  // by default c is immutable
    // c=90;
    //
    // let mut v=vec![1,2,3,4];
    // v.push(20);
    // // arrays
    // // let arr:[i32;4]=[1,2,3,4];


    // ownership
    // rust says every memory allocation (value) that you create will always  have a single owner
    // and when that owner goes out of scope or its lifetime is finished ,what rust does is drops it
    // rust has inbuilt mechanism of clearing the memory which we call as drop semantics

   //  let mut c=String::from("john");  // Vec<u8>
   //  //c=String::from("don");  // the value john from the heap gets dropped
   //  // c.push('h');
   // let y=c;  // the value stored in c gets moved (or gets shifted to) y
   // c variable gets dropped

    //println!("{}",c);

    // anything that is stored in heap and its reference is stored on stack  is moved
    //
    // let v=vec![1,2,3,4,5];
    // some_fn(v);
    // println!("{:?}",v)


    // Copy
    // Any data that has a fixed or known at compile time ,will be stored in stack
    // and it follows the Copy trait
    // its value get copied

    // let j=8;
    // let k=j;
    // // let j:i128=
    // 
    // println!("{}",j);
    // println!("{}",k);
    // 
    // param(j);
    // println!("{}",j);

    
    // // Clone   // is also a trait which lets heap stored data types to be cloned
    // let v=vec![1,2,3,4,5];
    // let k=v.clone();
    // let h=k.clone();
    // some_fn(k); // a new memory allocation on heap is made and its reference is assigned to the fn args
    // println!("{:?}",v);
    // println!("{:?}",h);
    
    
    // Borrowing
    // Borrowing happens by using references
    // two types of borrowing mutable and non mutable
    
    // Immutable borrowing 
    // let c=String::from("john");
    // // modify_str(&c);  // we are borrowing the value 
    // // println!("{}",c);
    // 
    // let v=&c;  // gets copeid internally // address of reference borrowed
    
    // //  Mutable borrowing
    // let  mut v =vec![1,2,2,3,4];  
    // let mv=&mut v;
    // 
    // mv.push(9);
    // println!("{:?}",mv);  // debug trait 
    
    // There are some rules of borrowing
    // You can have n number of immutable borrowing

    let  mut v =vec![1,2,2,3,4];
    // let mv=&v;
    // 
    // mv.push(9);
    // println!("{:?}",mv);  // debug trait 
    // print_vec(&mv);
    // print_vec(&mv);
    // print_vec(&mv);
    // print_vec(&mv);
    // print_vec(&mv)
    
    // You can have only single mutable reference

    let  mut v =vec![1,2,2,3,4];
     let mv=& mut v;
    mv.push(9);
    let mv2=& mut v;
    
   // mv2.push(90);
// so if you have  multiple borrowed reference and if you are using first reference even after you have created 
    // the secodn reference ,then rust compiler throws error 
    // other wise if there is no usage of the first borrorwed reference
    // it will automactally infer that the  first reference has been dropped 
    // and the second mutable reference can be used
    //print_vec(mv);  
    
    
    // you can have either a mutable borrowing or immutable borrowing ,they cannot exist together

    let  mut v =vec![1,2,2,3,4];
    // let mv1=& mut v;
    // mv1.push(9);
    // let mv2=&v;  // immutable reference 
    // 
    // mv1.push(9);

    let a="hello";
    let b="hello2";
    
    // let x=longest(a,b);
  // let u= foo();
    
}

// fn longest <'a>(v1:&'a str,v2:&'a str)->&'a str{
//     if v1.len() >v2.len(){
//         v1
//     }else{
//         v2
//     }
//     
// }


fn foo()->String{
   let y= String::from("hello");
    y
}




fn modify_vec(v:& mut Vec<i32>){
    v.push(-9);
}

fn modify_str(v:&String){  // here we are passing the borrowed value
     println!("{}",v)
}

// fn param(c:i32){
//     println!("{}",c)
// 
// }
// 
// fn  some_fn(vc:Vec<i32>){
//     println!("{:?}",vc)
// }




