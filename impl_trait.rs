use std::iter;
use std::vec::IntoIter;
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
    .map(|line|{
        line.map(|line|{
            line.split(',')
            .map(|entry| String::from(entry.trim()))
            .collect()
        })
    })
    .collect()
}
// parse_csv_document 是泛型的，允许它取任何实现了 BufRead 的类型，比如 BufReader<File> 或 [u8]，但是 R 是什么类型并不重要，R 只是用来声明 src 的类型，所以函数可以也可以写成：
//也可以写成
fn parse_csv_document2(src:impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
    .map(|line|{
        line.map(|line|{
            line.split(',')
            .map(|entry| String::from(entry.trim()))
            .collect()
        })
    })
    .collect()
}

// 请注意，使用 impl Trait 作为参数类型意味着您无法明确说明您使用的函数形式，即 parse_csv_document::<std::io::Empty>(std::io::empty()) 将不起作用第二个例子。


//2 . as a return type
//如果你的函数返回一个实现了 MyTrait 的类型，你可以把它的返回类型写成 -> impl MyTrait。这可以大大简化您的类型签名！




// 这个函数组合了两个 `Vec<i32>` 并返回一个迭代器。
// 看看它的返回类型多复杂！
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u:Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>,IntoIter<i32>>> 
{
    v.into_iter().chain(u.into_iter()).cycle()

}


// 这是完全相同的函数，但它的返回类型使用 `impl Trait`。
// 看看它是多么简单！
fn combine_vecs (v: Vec<i32>,u: Vec<i32>,) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![4,5];
    let mut v3 = combine_vecs(v1,v2);
    assert_eq!(Some(1),v3.next());
    assert_eq!(Some(2),v3.next());
    assert_eq!(Some(3),v3.next());
    assert_eq!(Some(4),v3.next());
    assert_eq!(Some(5),v3.next());
    println!("all done");
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2),3);

    let singles = vec![-3,-2,2,3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(),vec![4,6])
}



///更重要的是，有些 Rust 类型是写不出来的。
/// 例如，每个闭包都有自己未命名的具体类型。
/// 在 impl Trait 语法出现之前，您必须在堆上进行分配才能返回闭包。
/// 但现在您可以静态地完成所有操作，如下所示：
fn make_adder_function(y:i32) -> impl Fn(i32) -> i32
{
    let closure = move |x: i32| {x+y};
    closure
}

///您还可以使用 impl Trait 返回一个使用 map 或 filter 闭包的迭代器！
/// 这使得使用 map 和 filter 更容易。
/// 因为闭包类型没有名称，所以如果您的函数返回带有闭包的迭代器，则您不能写出显式返回类型。
/// 但是使用 impl Trait 你可以很容易地做到这一点：
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
    .iter()
    .filter(|x| x > &&0)
    .map(|x| x* 2)
}