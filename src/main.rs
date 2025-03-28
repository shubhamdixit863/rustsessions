use std::thread;

fn main() {
    
    let main_var=100;
    
    let handles:Vec<_>=(0..4).map(|i|{
        thread::spawn( move || {
            let local_thread_var=1*10;
            let heap_var=Box::new(i*100);
            
            println!("Thread {} heap var address :{:p}",i,&heap_var);
            println!("Thread {} stack var address :{:p}",i,&local_thread_var);
            
            
        })
    }).collect();
    
    
    

}


// Compilation   

// rust llvm  ---->converts into a native binary
// --this native binary contains -->main function ,rust runtime setup ,symbols and static data

