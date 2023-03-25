use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

struct List(Vec<i32>);

fn main() {
    println!("hello world");

    /*
    格式化打印
    */
    println!("{} days", 31);

    /*
    多个占位符号
    */
    println!(
        "指定变量: 变量1:{0}, 变量2:{1} , 变量1:{0}",
        "我是变量1", "我是变量2"
    );

    /*
    指定变量名
     */
    println!(
        "指定的变量名格式化:{hello}, 我的年龄是{age}",
        hello = "梁典典",
        age = 22
    );

    /*
    指定进制格式化
    */
    println!("十进制(默认): {}", 1000);
    println!("二进制: {:b}", 1000);
    println!("八进制: {:o}", 2000);
    println!("十六进制(小写): {:x}", 1000);
    println!("十六进制(大写): {:X}", 1000);

    /*
    打印:  指定空格宽度:            1000
     */
    println!("指定空格宽度(打印的间隔): {number:>15}", number = 1000);

    /*
    填充数字:19999

    不满足向后填充额外的数字
     println!("填充数字:{number:9<5}", number = 100);
    填充数字:10099
    */
    println!("填充数字:{number:9<5}", number = 100);

    /*
     向前面添加额外的数字
    width$: 引用变量
     */
    println!("填充数字高级玩法: {number:0>width$}", number = 1, width = 7);

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");

    println!("------------------------------------");

    //debug

    println!("{:?} months in a year.", 12);

    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    println!("------------------------------------");

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);

    println!("------------------------------------");

    let minmax = MinMax(0, 14);

    println!("比较两种打印结构");
    println!("display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {big} and the smail is {small}",
        big = big_range,
        small = small_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("------------------------------------");

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?
        }
        write!(f, "]")
    }
}
