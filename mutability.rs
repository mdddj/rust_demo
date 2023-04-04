/*
可变数据可以使用&mut T进行可变借用。
这被称为可变引用，并为借用者提供读/写访问权限。
相比之下，&T通过不可变引用借用数据，借用者可以读取数据，但不能修改数据：
*/

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    //`&“static str”是对只读内存中分配的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

//此函数引用一本书
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

//此函数引用了一本可变的书，并将“年份”更改为2014年
fn new_edition(book: &mut Book) {
    book.year = 2014;
    book.title = "hello.";
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

///main函数
fn main() {
    //创建一个名为`immutabook的不可变图书`
    let immutabook = Book {
        //字符串文字的类型为“&'static str`
        author: "liang diandian",
        title: "About Rest",
        year: 1979,
    };

    //创建一个可变的副本
    let mut mutabook = immutabook;
    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    //下面这句会报错: 错误不能将不可变对象作为可变对象借用
    // new_edition(&mut immutabook);
}
