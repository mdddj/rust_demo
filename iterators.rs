struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci
{
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    
    Fibonacci {curr: 0,next: 1}
}

fn main() {
    let mut sequence = 0..3;
    println!("Four consecutive next calls on 0..3");
    println!("> {:?}",sequence.next());
    println!("> {:?}",sequence.next());
    println!("> {:?}",sequence.next());
    println!("> {:?}",sequence.next());
    
    println!("iterate through 0..3 using for");
    for i in 0..3 {
        println!("> {}",i);
    }
    
    println!("the forst fpr terms of the Fibonacci sequence are:");

    for i in fibonacci().take(4) {
        println!("> {} ",i);
    }
    
    println!("the next four terms of the fibonacci sequence are : ");
    for i in fibonacci().skip(4).take(4){
        println!("> {}",i);
    }
    
    let array = [1u32,3,3,7];
    println!("iterate the following array {:?}", &array);
    
    for i in array.iter(){
        println!("> {}",i);
    }
    
}