fn create_box() {
    //在堆上分配一个整数

    let _box1 = Box::new(3i32);

    //“_box1”在此处被销毁，内存被释放
}

fn main() {
    //在堆上分配一个整数
    let _box2 = Box::new(5i32);
    {
        //在堆上分配一个整数
        let _box3 = Box::new(4i32);
        //`_box3`在这里被销毁，内存被释放
    }

    //制作许多盒子只是为了好玩
    //没有必要手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }

    //`_box2`在这里被销毁，内存被释放

    let x = ToDrop;
    println!("mode a to drop");
    println!("============================");

    //在堆上分配一个整数
    let x = 5u32;

    //将x复制到y中,不移动任何资源
    let y = x;
    println!("x is {}, and y is {}",x,y);

    let a = Box::new(5i32);
    //`a`是指向heap_分配的整数的指针
    println!("a contains: {}",a);

    //将a移动到b中
    //a的指针地址被复制到b中,(不是地址)
    //现在两者都是指向相同堆分类数据的类型指针,但是现在b现在拥有它
    let b = a;

    //后面不再再使用a的数据,因为它不再拥有它
    // code>>:  println!("视图调用a,可能会报错, {}",a);
    // 上面这个句会报错,原因是,a的所有权被转移到了b,不能再使用它了.
    // 报错信息: value borrowed here after move

    // heap: 翻译过来是: 堆内存


    //执行并进入这个函数,相等于这个函数拥有了b的所有权
    //rust的解释: 这个函数将会从b获取分配堆内存的所有权
    destroy_box(b);
    //执行完上面这个函数,main函数后面就不能使用b这个变量了.


    // 可以尝试打印一下b,会报错,编译也不通过.
    // println!("b value is {} ", b);



    println!("可变性::::");


    //解释: 当所有权转移时,数据的可变性可以改变


    let immutable_box = Box::new(5u32);
    println!("immutable box contains {}", immutable_box);

    //下面会编译报错: 无法分配给`*immutable_box`，因为`immutable_box`未声明为可变
    // *immutable_box = 4;

    //将immutable_box变量的所有权移动给mutable_box, 并设置为可变性,可以更改
    let mut mutable_box = immutable_box;

    println!("mutable box contains {}", mutable_box);

    //这里就可以更改变量的值了
    *mutable_box = 4;
    // mutable_box = 8;

    println!("mutable box contains {}",mutable_box);


    println!("部分动作,在单个变量的解构中,可以同事使用 by-move和模式绑定,by-reference这样做将导致变量的部分移动,这意味着变量的一部分将被移动而其他部分将保留,在这种情况下,父变量之后不能做为一个整体使用,但仅被引用(未移动)的部分仍然可以使用.");


    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {name: String::from("梁典典"),age: Box::new(22),};


    // name 从 person 中移动出来了,但是age是被引用的
    let Person {name,ref age} = person;

    println!("用户的年龄是:{}",age);
    println!("用户的名字是:{}",name);

    println!("用户的年龄是(从person中)取出来.{}",person.age);


    //因为它的部分值被移动
    //println!("person is {:?}",person); //这句话会报错:value borrowed here after partial move
}






// 这个函数可以获得堆内存中的所有权
fn destroy_box(c: Box<i32>) {
    println!("destorying a box that contains {}", c);
}

//自定义析构函数
struct ToDrop;

impl Drop for ToDrop {

    ///这一行会在函数执行完最后一行释放
    fn drop(&mut self) {
        println!("to drop being dropped");
    }
}
