#![allow(dead_code)]

///Sushi-寿司
#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

//是否有原料,如果是寿司,则提示没有原料,做不了寿司
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

//是否有配方,除了CordonBleu,其他什么都有
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

//做一道菜,我们需要菜谱和原料
//这里用的是match来做判断
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

// 这里可以使用and_then更紧凑的重写
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

/// flatten 可以将Option<Option<Food>> 类型简化为Option<Food>类型
/// 在这个上下文中， flatten 函数用于将嵌套的 Option 类型展平为单层的 Option 类型。具体来说， flatten 函数会将 Option<Option<T>> 类型的值转换为 Option<T> 类型的值。
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("哇,在{:?}中我们将会吃{:?}", day, food),
        None => println!("哦不,我们不能吃这个,{:?}", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
