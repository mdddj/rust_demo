
// 这个函数获得盒子的所有权,并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("销毁盒子:{}",boxed_i32);
}

//这个函数借用了i32的值
fn borrow_i32(borrowed_i32: &i32){
    println!("this int is: {}",borrowed_i32);
}

fn main() {
    //创建盒子类型的i32
    let boxed_i32 = Box::new(5_i32);
    //创建存放在堆内存的i32
    let stacked_i32 = 6_i32;
    //借用盒子的值,未取得所有权
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        //再次引用box的值
        let _ref_to_i32 : &i32 = &boxed_i32;
        eat_box_i32(boxed_i32);

        //内部值销毁后尝试调用值,下面这句会报错.
//         borrow_i32(_ref_to_i32);
    }
    //下面这句话也会报错
//     eat_box_i32(boxed_i32);
}