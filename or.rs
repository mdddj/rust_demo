#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon
}
fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let not_fruit: Option<Fruit> = None;
    let first_available_fruit = not_fruit.or(orange).or(apple);
    println!("first available fruit : {:?}",first_available_fruit);

}
