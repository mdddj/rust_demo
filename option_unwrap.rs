///使用match可以很好的控制对应的逻辑
fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("喝柠檬水"),
        Some(inner) => println!("{}? 太棒了", inner), //inner表示输入的任何值,使用这个变量来接收他
        None => println!("没有喝东西吗?"),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap(); //如果drink为none,会触发恐慌thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', option_unwrap.rs:10:24
    if inside == "lemonade" {
        panic!("啊啊啊啊啊啊喝了柠檬水");
    }
    println!("太棒了,,这个,{}", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;
    drink(coffee);
    drink(nothing);
}
