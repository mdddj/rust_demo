use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a,T: 'a>(&'a T);

fn print<T>(t: T) where T: Debug {
    println!("print t is {:?}",t);
}

fn print_ref<'a,T>(t: &'a T) where T: Debug + 'a {
    println!("print_ref t is {:?}",t);
}

fn main() {
    let x = 7;
    let refx = Ref(&x);
    print_ref(&refx);
    print(refx);
}