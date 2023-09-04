/// 恐慌

/// lemonade : 柠檬水
///
/// 喝东西函数
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("条件符合了恐慌执行条件. {}", beverage);
    }
    println!("程序正常执行");
}

fn main() {
    drink("water"); //喝水
    drink("lemonade"); //喝柠檬水
}
