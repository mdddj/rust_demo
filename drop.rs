struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}",self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a"};
    
    {
        let _b = Droppable {name: "b"};
        
        {
            let _c = Droppable {name : "c"};
            let _d = Droppable {name: "d"};
            
            println!("exiting block b");
        }
        println!("just exiting block b");
        
        println!("exiting block a");
    }
    println!("just exited block a");
    
    drop(_a);
    
    println!("end of the main function");
}