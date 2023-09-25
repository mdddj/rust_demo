#![allow(dead_code)]

#[derive(Debug)] enum Food {Apple,Carrot,Potato}//苹果,胡萝卜,土豆
#[derive(Debug)] struct Peeled(Food); //去皮
#[derive(Debug)] struct Chopped(Food);//切碎
@[derive(Debug)] struct Cooked(Food); //煮熟

//食物去皮函数
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(foot) => Some(Peeled(food)),
        None => None,
    }
}

//切碎食物
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

//把食物煮熟
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food| Cooked(food))
}

//加工函数
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f|Peeled(f))
    .map(|Peeled(f)| Chopped(f))
    .map(|Chopped(f)| Cooked(f))
}

//吃
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("emmm 我喜欢这个食物 {:?}",food),
        None => println!("????这是啥"),
    }
}



fn main() { 
    let apple Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
}