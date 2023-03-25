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
    println!("视图调用a,可能会报错, {}",a);
    // 上面这个句会报错,原因是,a的所有权被转移到了b,不能再使用它了.
    // 报错信息: value borrowed here after move

    // heap: 翻译过来是: 堆内存


    //执行并进入这个函数,相等于这个函数拥有了b的所有权
    //rust的解释: 这个函数将会从b获取分配堆内存的所有权
    destroy_box(b);
    //执行完上面这个函数,main函数后面就不能使用b这个变量了.

}

// 这个函数可以获得堆内存中的所有权
fn destroy_box(c: Box<i32>) {
    println!("destorying a box that contains {}", c);
}

//自定义析构函数
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("to drop being dropped");
    }
}
