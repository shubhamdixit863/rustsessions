use std::thread;



// mutability ,ownership and drop semantics ,lifetimes
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

    
    // Clone   // is also a trait which lets heap stored data types to be cloned
    let v=vec![1,2,3,4,5];
    let k=v.clone();
    let h=k.clone();
    some_fn(k); // a new memory allocation on heap is made and its reference is assigned to the fn args
    println!("{:?}",v);
    println!("{:?}",h);


}

fn param(c:i32){
    println!("{}",c)

}

fn  some_fn(vc:Vec<i32>){
    println!("{:?}",vc)
}




