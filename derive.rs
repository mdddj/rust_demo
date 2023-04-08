
/**
#[derive(PartialEq, PartialOrd)]是一个Rust的派生属性（derive attribute），它可以自动为一个结构体或枚举类型实现PartialEq和PartialOrd trait。PartialEq trait用于比较两个值是否相等，PartialOrd trait用于比较两个值的大小关系。使用#[derive(PartialEq, PartialOrd)]可以简化代码，避免手动实现PartialEq和PartialOrd trait，同时也可以确保实现的正确性。当结构体或枚举类型需要进行比较时，只需要在定义时加上#[derive(PartialEq, PartialOrd)]即可。需要注意的是，只有所有成员都实现了PartialEq和PartialOrd trait，才能为结构体或枚举类型自动实现这两个trait。如果有成员没有实现这两个trait，编译器会报错。
这段代码定义了三个结构体：Centimeters、Inches和Seconds。Centimeters和Inches分别表示厘米和英寸，Seconds表示秒。Centimeters结构体包含一个f64类型的值，Inches结构体包含一个i32类型的值。在Inches结构体中，定义了一个方法to_centimeters，该方法将英寸转换为厘米，并返回一个Centimeters结构体。在方法中，首先使用模式匹配将self解构为&Inches(inches)，然后将英寸转换为厘米，并返回一个Centimeters结构体。在main函数中，首先定义了一个Seconds结构体的实例_one_second，然后定义了一个Inches结构体的实例foot，并打印出来。接着定义了一个Centimeters结构体的实例meter。然后使用if语句比较一英尺和一米的大小，并将结果存储在变量cmp中。最后打印出结果。
**/
#[derive(PartialEq,PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);


fn main() {
    let _one_second = Seconds(1);
    let foot = Inches(12);
    println!("One foot equals {:?}",foot);
    let meter = Centimeters(100.0);
    
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    }else {
        "bigger"
    };
    println!("one foot is {} than one meter.",cmp);
}

