#[derive(Clone,Copy)]
struct Point {x: i32, y: i32}

/// ref 的基本使用
fn main() {
    let c = 'Q';
    
    
    //ref 等同于 &c
    let ref  ref_c1 = c;
    let ref_c2 = &c;
    
    println!("ref_c1 equals ref_C2 : {}",*ref_c1 == *ref_c2);// true
    
    let point = Point {x: 0, y: 0};
    
    /**
    
    这段 Rust 代码首先定义了一个名为 _copy_of_x 的变量。这个变量的类型是 i32，但是它没有被赋值。

接下来，代码创建了一个名为 Point 的结构体，该结构体包含 x 和 y 两个字段，类型分别为 i32。这个结构体被定义在其他地方，并且没有在这个代码段中给出。

接着，代码从 point 变量中解构出 x 的引用，而且 y 的值被忽略了。这个引用被保存在名为 ref_to_x 的变量中。注意到解构语句中的 ref 关键字，它表示我们希望得到的是一个指向 point.x 的引用，而不是对 point.x 的拷贝。这种引用称为 “ref 引用”。

最后，代码返回 ref_to_x 所引用的值，并将其解引用，得到 x 的拷贝。这个拷贝被赋值给 _copy_of_x 变量。由于 _copy_of_x 是一个不使用的变量，所以它的值在赋值之后就被丢弃了。

因此，这段代码的目的是从一个给定的结构体 Point 中获取 x 字段的值，并将其拷贝到一个新的变量中。
    */
    let _copy_of_x = {
        let Point {x: ref ref_to_x,y: _} = point;
        *ref_to_x
    };
    
    let mut mutable_point = point;
    
    {
        let Point {x: _, y: ref mut mut_ref_to_y} = mutable_point;
        *mut_ref_to_y = 1;
    }
    
    println!("point 变量的值是 x={}, y={}",point.x,point.y);
    println!("mutable_point 的值是: x={}, y={}",mutable_point.x,mutable_point.y);
    
    
    let mut mutable_tuple = (Box::new(5u32),3u32);
    
    {
        let (_,ref mut last)  = mutable_tuple;
        *last = 2u32;
    }
    
    println!("tuple is {:?}",mutable_tuple);
    
    
    
}
