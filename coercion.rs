fn multiply<'a>(first: &'a i32,second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32,_: &'b i32) -> &'b i32 {
    first
}
fn main() { // a
    
    let first = 2;
    { //b
        let second = 3;
        {//c 
        
        }
        println!("the product is {}",multiply(&first,&second));
        println!("{} is the first ",choose_first(&first,&second));
        
    }
}