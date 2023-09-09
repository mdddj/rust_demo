///使用不同的恐慌策略来执行不同的代码行
/// 在编译的时候使用 -C panic="abort" 或者 panic="unwind"

/// 翻译过来就是abort: 使流产；中止(尤指很可能失败的事情)；(使)夭折流产；放弃；未成形的计划；未做好的打算
/// unwind: 展开,放松
/// &str: 应该是一个字符串切片

///来自官网的翻译:
/// 上一节说明了错误处理机制死机。可以根据panic设置有条件地编译不同的代码路径。当前可用的值为展开和中止。
///在前面柠檬水示例的基础上，我们明确地使用恐慌策略来执行不同的代码行。

fn drink(beverage: &str) {
    //如果喝的是柠檬水
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("严格的,放弃执行");
        } else {
            println!("继续执行吧,请继续下面的代码");
        }
    } else {
        println!("我要喝点什么东西....{}", beverage);
    }
}

///编译:  rustc abort_and_unwind.rs -C panic=abort
///输出:   ./abort_and_unwind
///我要喝点什么东西....water
///严格的,放弃执行
///
///
/// 如果换成 -C panic=unwind 宽松的编译,会输出"继续执行吧,请继续下面的代码"
fn main() {
    drink("water");
    drink("lemonade")
}

///还有另外一种方式,是使用注解的形式

#[cfg(panic = "unwind")]
fn ah() {
    println!("我是宽松的代码执行函数");
}

/// 注意,这里使用的是not(panic="unwind")
#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("停止吧!");
}

fn drink2(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!(".....喝东西.....:{}", beverage);
    }
}
