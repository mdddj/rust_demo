#[derive(Debug,Clone,Copy)]
struct Unit;

#[derive(Clone,Debug)]
struct Pair(Box<i32>,Box<i32>);

fn main() {
    let unit = Unit;
    let copied_unit = unit;

    println!("原来的:{:?}",unit);
    println!("copy: {:?}",copied_unit);


    let pair = Pair(Box::new(1),Box::new(2));
    println!("原来的:{:?}",pair);

    let moved_pair = pair;
    println!("移动的:{:?}",moved_pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    println!("克隆:{:?}",cloned_pair);
}