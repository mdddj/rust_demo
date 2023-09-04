
#![allow(dead_code)]

//1.panic
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAAaaaa!");
    }
    println!("some refresing {} is all i need!",beverage)
}

//2.abort&unwind
//这段代码定义了一个函数 `drink2`，该函数接收一个字符串类型的参数 `beverage`，并根据不同的参数做出不同的响应。如果 `beverage` 的值是字符串 "lemonade"，那么程序会判断当前是否启用了 Panic Abort，如果启用了，那么程序会输出 "this is not your party, Run!!!"，否则输出 "spit it out !!!!"。如果 `beverage` 的值不是 "lemonade"，则程序会输出 "some refreshing [beverage] is all i need!"。在 `main` 函数中，分别调用 `drink2` 函数，传入参数 "water" 和 "le "lemonade"。
fn drink2(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic="abort"){println!("this is not your party, Run!!!");}
        else{
            println!("spit it out !!!!");
        }
    }else{
        println!("some refreshing {} is all i need!",beverage)
    }

}
#[cfg(panic="unwind")]
fn ah() {
    println!("spit it out !!!");
}

#[cfg(not(panic="unwind"))]
fn ah() {
    println!("This is not you part, Run!!!!");
}

// rustc main.rs -C panic=abort 会执行 This is not you part, Run!!!!
// rustc main.rs 会执行 spit it out !!!
fn drink3(beverage: &str){
    if beverage == "lemonade" {ah();}
    else{
        println!("Some refreshing {} is all i need",beverage);
    }
}



// 18.3 option & unwrap
fn give_adult(drink: Option<&str>){
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice.",inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink4(drink: Option<&str>){
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }
    println!("I love {}s !!!!",inside);
}


// 18.4 Unpacking options with ?
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? +  1;
    Some(format!("Next year I will be {}",next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Copy, Clone)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}


impl Person {
    fn work_phone_area_code(&self) -> Option<u8>{
        self.job?.phone_number?.area_code
    }
}

//18.3.2 combinators: map 组合器
#[derive(Debug)] enum Food {Apple,Carrot,Potato}
#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);


fn peel(food: Option<Food>) -> Option<Peeled> {
    match food  {
        Some(food) => Some(Peeled(food)),
        None => None
    }
}

// 切食物。如果没有，则返回“None”。
// 否则，返回切碎的食物。
fn chop(peeled: Option<Peeled>) -> Option<Chopped>
{
    match peeled
    {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

//煮食物。在这里，我们展示了 `map()` 而不是 `match` 来处理案例。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}
// 一个按顺序去皮、切碎和烹饪食物的函数。
// 我们链接了 `map()` 的多次使用以简化代码。
fn process(foot:Option<Food>) -> Option<Cooked>{
    foot.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
}


//尝试吃之前检查是否有食物！
fn eat(food: Option<Cooked>){
    match food {
        Some(food) => println!("Mmm,I love {:?}",food),
        None => println!("哦不，这东西我不吃")
    }
}


fn main() {
    // drink3("water");

    // drink3("lemonade");
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;
    drink4(coffee);
    drink4(nothing);
    println!("--------------18.3.1 ? 可以理解为空安全");
    let p = Person {
        job: Some(Job{
            phone_number: Some(PhoneNumber{
                area_code: Some(61),
                number: 439922222
            })
        })
    };
    assert_eq!(p.work_phone_area_code(),Some(61));
}
