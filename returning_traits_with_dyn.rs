struct Sheep {}
struct Cow{}

trait Animal {
    fn noise(&self) -> &'static str;
}

///为小绵羊实现manial接口
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

fn random_anial(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep{})
    }else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_anial(random_number);
    println!("你随机生成了一只动物,上面写着{}",animal.noise());
}