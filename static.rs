use std::fmt::Debug;
//使一个具有“静态”生存期的常量。
static NUM: i32 = 18;

//返回对“NUM”的引用，其中它的“”是静态的`
//生存期被强制为输入参数的生存期。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}



fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is : {:?}", input);
}

fn main(){
    {
        //将“字符串”设为文字并打印：
        let static_string = "I am in read-only memory";
        println!("static_String : {}",static_string);
        //当“static_string”超出范围时，引用
        //无法再使用，但数据仍保留在二进制文件中。
    }
    
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coreced_static : {}",coerced_static);
    }
    
    println!("NUM保持可访问状态: {}",NUM);
    
    //i是被拥有的，不包含任何引用，因此它是“静态的：
    let i = 5;
    print_it(i);
    
    //哎呀，我只有由的范围定义的生存期
    //main（），所以它不是“静态的：
    print_it(&i);
}