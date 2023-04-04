/**
数据可以被多次不可变地借用，但在不可变地借入时，原始数据不能被可变地借用。
另一方面，一次只允许一次可变借款。只有在上次使用可变引用之后，才能再次借用原始数据。
*/

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let anothor_borrow = &point;

    //通过引用和原始所有者访问数据
    println!(
        "对象的坐标为: ({},{},{})",
        borrowed_point.x, anothor_borrow.y, point.z
    );

    //这句会报错
    // let mutable_borrow = &mut point;

    println!(
        "对象的坐标: ({},{},{})",
        borrowed_point.x, anothor_borrow.y, point.z
    );

    // ?? 无法理解为什么放上面会报错,放这里就没事???
    //不可变引用不再用于代码的其余部分，因此
    //可以使用可变引用重新调用。
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 这里教程上说会报错,但是我编译是不会报错的????
    // let y = &point.y;

    // println!("point z coordinate is {}", point.z); //报错,此处发生不可变借用

    println!(
        "point has coordinates: ({},{},{})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let new_borrowed_point = &point;

    println!(
        "Point now has coordinates: ({},{},{})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
