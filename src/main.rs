use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::ErrorKind;
use std::os::unix::raw::ino_t;
use std::thread::sleep;
use crate::custom_mod::MyView;

pub mod custom_mod;

mod front_of_house2 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house2::hosting;


fn main() {
    hosting::add_to_waitlist();
    custom_mod::eat_at_restaurant();

    println!("Hello, world!");
    let mut a = 10;
    a = 20;
    let mut y = "3434";
    y = "f3fr3r";

    println!("{a} {y}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let a = 10;
    let b = 20.0;

    let c = '2';
    let d: char = '王';
    let e = '😄';
    println!("{a} {b} {c} {d} {e}");

    let t: (f32, f64, u32) = (500.0, 6.4, 1); // 三个元组
    let t1 = t.0; // 通过数字下标来访问元组
    let t2 = t.1;
    let t3 = t.2;

    println!("t: {t1} {t2} {t3}");
    println!("t: {}", t.2);


    // 数组与元组不同的是，数组里面的元素必须是同一个类型，例如：
    let a: [i32; 4] = [1, 2, 3, 4]; // 表示一个 int 32 位，长度是 4 的数组
    let a = [1, 2, 3, 4]; // 简写：表示一个 int 32 位，长度是 4 的数组
    let a = [4; 5]; // 简写 [value; length] ：表示一个 int 32 位，长度是 5， 并且每一个数字都是 4
    let size = a.len();
    println!("{}, {size}", a[0]); // 通过下标访问某个元素

    another_function();

    let a = 10;
    let b = 30.0;
    print_params(a, b);

    let mut a = 10;
    let b = { // 注意此时 b 没有任何类型
        a = 100;
        a + 10; // 末尾加上了分号，则没有返回任何数据
    };
    let b = {
        a = 100;
        a + 10 // 没有加分号，则这个值就是 b 的值
    };
    println!("{b}");

    let b = test_return();
    println!("{b}");

    let a = 10;
    if a == 10 { // if
        println!("true");
    } else if a > 10 { // else if
        println!("20")
    } else { // else
        println!("30")
    }
    let b = if a == 10 { 2 } else { 30 };

    loop { // 类似于 do .. while
        println!("100");
        break;
    }


    let a = loop { // 类似于 do .. while
        println!("100");
        break 10; // 意思是返回最后返回一个 10
    };

    // 字面量：字面量是不可以修改的
    let mut a = "1234"; // &str  字面量
    let b = a;
    a = "456";
    println!("{}, {}", a, b); // 输出 456， 1234

    // String 与字面量不同，String 是可以修改的
    let mut a = String::from("1234"); // String
    a.push_str(" 567");
    println!("{}", a); // 输出 1234 567， String 是可以修改的

    let s1 = String::from("abc");
    let s2 = s1;
    // println!("{}", s1); 因为 s1 的值被借走了

    let s1 = String::from("ttt");
    borrow_value(s1);
    // println!("{}", s1);  报错，因为 s1 把值借给 函数 borrow_value 了


    let mut s = String::from("hello"); // s 可变
    change(&mut s); // 引用 reference 传递
    println!("{}", s);

    let mut s = String::from("hello"); // s 可变
    let mut s1 = &mut s;
    let mut s2 = &mut s;
    // println!("{}", s1); // 报错，因为 s 又借给了s2, 所以 s1 没有值
    println!("{}", s2); // 正确，最后一个引用可以访问值

    let mut s = String::from("hello"); // s 可变
    let mut s1 = &s;
    let mut s2 = &s;
    let mut s3 = &mut s;
    // println!("{} {} {}", s1, s2, s3); // 报错，因为 s1 s1 是不可变引用 s3 是可变的，同一时间只能有一个


    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {} {}", word, hello, world);

    // s.clear(); // error, 因为 s 已经借了，所以无法执行 clear 操作
    // println!("{} {} {}",word, hello, world);


    let mut user1 = User {
        id: 10,
        email_address: String::from("abc@gmail.com"), // 顺序可以和结构体的顺序不同
        name: String::from("William"),
        password: String::from("123455"),
    };

    user1.id = 20;
    println!("{}", user1.name);
    println!("{}", user1.password);

    let mut user2 = User {
        name: String::from("siyehua"),
        ..user1 // 被复制的对象必须卸载最后面
    };
    println!("{}", user1.name);
    println!("{}", user2.id);
    // println!("{}", user1.password);// 报错，因为 password 的值已经借给 user2 了

    let tuple1 = MyTuple1(10, 20.0, String::from("123234"));
    let tuple2 = MyTuple2(10, String::from("123234"), 20.0);
    println!("{}", tuple1.0);


    let myUnitLike = MyUnitLike;

    let mylocation = MyLocation {
        x: 10,
        y: 20,
        name: String::from("William"),
    };
    println!("{:#?}", mylocation);
    println!("{:?}", mylocation);
    dbg!(&mylocation);

    mylocation.show_location();
    let mylocation = MyLocation::create_same(40);
    println!("x {} y {}", mylocation.x, mylocation.y);

    let a = Woman {
        name: String::from("Lisa"),
    };
    let b = Man {
        name: String::from("Ben"),
    };

    let a = Person::MAN(String::from("Ben"));
    let b = Person::WOMAN(String::from("Lisa"));

    let b = 100;
    let a = match b {
        10 => 2,
        20 => 30,
        other => 100,
    };
    println!("{a}");

    let b = 0;
    if let a = b {
        println!("if let {a}");
    } else {
        println!("else {a}");
    }

    let some_option: Option<i32> = Some(42);
    if some_option.is_some() {
        println!("Option contains the value: {}", some_option.unwrap());
    } else {
        println!("Option is None");
    }

    let some_option: Option<i32> = Some(42);
    // 使用 if let 解构 Option 枚举
    if let Some(number) = some_option {
        println!("Option contains the value: {}", number);
    }

    // 简单的使用
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    // let v = vec![1, 2.0, 3]; // 编译会报错，2 是 float 类型，不能与 i32 混装

    // 添加
    // v.push(2.0);// 报错，2.0 不是 i32 为，不可添加
    v.push(100);
    v.push(1);

    // 移除
    v.remove(0);
    v.pop();// 移除最后一个元素;

    // 访问单个
    let value = &v[2]; // 使用 index 下标进行读取
    println!("value:{value}");
    let value: Option<&i32> = v.get(2); // 使用 get 方法读取
    match value {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element."),
    }

    // 循环取出
    for n_ref in &v {
        // 这种方式取出的引用是不可变的 immutable, 这里的打印 1，,2 没有任何区别，是因为 v 里面的是 i32， 基本类型
        let a = n_ref + 1;
        println!("{a}");
        let a = *n_ref + 1;
        println!("*{a}");
        println!("for value: {n_ref}");
    }
    for n_ref in &mut v {
        // let a = n_ref + 1; // 此时 n_ref 的类型是 &mut 32 ，不能直接修改，会报错
        // println!("{a}");
        let a = *n_ref + 1;
        println!("*{a}");
        println!("for value: {n_ref}");
    }

    // 可变与不可变的区别：immutable vs mutable
    let mut v = vec![String::from("fef"), String::from("abc"), String::from("ttt")];
    for n_ref in &v {
        let a = n_ref; // a 的类型是 &String
        // n_ref.push_str("tttt"); // 这里会表看错，因为取出来的 n_ref 是不可变的
        println!("string vector:{}", n_ref);
    }
    for n_ref in &mut v {
        // n_ref 的类型是 &mut String
        n_ref.push_str("tttt");
        println!("string vector:{}", n_ref);
    }


    // let value = &v[20]; // 报错，超出了 index 下标
    // println!("value:{value}");

    let mut v = vec![1, 3, 4, 6];
    let mut v1 = &v[2]; // 不可变的
    v.push(3); // 可变的
    // 报错，一个 scope 中，不能同时存在不可变的引用和可变的引用
    // 因为 vector 的内存是连续的，添加一个新的元素(push), 可能导致重新分配内存，这两导致之前的 v1 指向的内存失效
    // print!("v1 value:{}", v1);

    // let mut v = Vec::new();
    // let s = String::from("Hello ");
    // v.push(s);
    // println!("original: {}", s);
    // println!("new: {}", v[0]);

    let t = "fefef";
    let mut a = String::new();
    a.push_str(t);
    a.push('f');
    let a = String::from(t);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3:{}", &s3);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = &s1 + &s2; // 无法编译，&String 没有重写 + 符号
    // println!("s3:{}", &s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + "-" + &s2;
    println!("s3:{}", &s3);

    let mut v = HashMap::new();
    v.insert("123", 4);
    v.insert("567", 4);
    v.remove("567");
    println!("{}", v.get("123").unwrap());
    // println!("{}", v.get("456").unwrap()); // 报错，567 已经被移除
    println!("{}", v.get("456").copied().unwrap_or(100)); // 输出 100

    for (key, value) in v {
        println!("key:{}, value:{}", key, value);
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // map.insert(field_name, field_value);// 直接穿 field_name 会导致 field_name 后面打印错误
    map.insert(&field_name, &field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("name:{}, value:{}", field_name, field_value);


    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(f) => f,
    //     Err(e) => panic!("open fail:{}", e.kind()), // 报错找不到文件
    // };
    // println!("open file success!");

    let file = File::open("hello.txt");
    if file.is_err() {
        if file.err().unwrap().kind() == ErrorKind::NotFound { // 判断错误类型
            println!("Can't found the file,so create it!");
            File::create("hello.txt").expect("create error");
        } else {
            panic!("open fail:{}", e)
        }
    }

    let mut vec: Vec<i32> = vec![1, 2, 3]; // vec:R,W,O
    let num: &i32 = &vec[2]; // vec:R,_,_ num:R,_,O *num:R,_,_
    for item in &vec { // item:R,_,_
        println!("value:{}", item);
    }
    println!("Third element is {}", *num);


    let v = MyType {
        x: 10,
        y: 10,
    };
    let a = v.f();
    let b = v.f2();
    println!("{}, {}", &a, &b);
    let a = "my";
    let c = return_it(&a);
    println!("{}", &c);

    let t = Teacher { name: "Lisa".to_string() };
    let c = Student { name: "Duo".to_string() };
    t.print_name();
    c.print_name();

    print_obj("have a try");
    print_obj(10);
    print_multiple(t);
    // print_multiple(c); // 报错，student 没有实现 Display，所以无法打印
}

// 可以不声明生命周期，会自动推断
fn longest3(x: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 有两个输入，无法自动推断
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

struct Teacher {
    name: String,
}

struct Student {
    name: String,
}

trait Human {
    // 没有实现
    fn get_name(&self) -> String;
    // 有默认实现
    fn print_name(&self) {
        println!("Your name is:{}", self.get_name())
    }
}

impl Human for Teacher {
    fn get_name(&self) -> String {
        format!("{}", self.name)
    }
}

impl Display for Teacher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.name.as_str(), f)
    }
}

impl Human for Student {
    fn get_name(&self) -> String {
        format!("{}", self.name)
    }

    fn print_name(&self) {
        println!("I a student, my name is:{}", self.get_name())
    }
}

// 限定
fn print_obj<T: Display>(value: T) {
    println!("{}", value);
}

// 多种组合
fn print_multiple2<T: Display + Human>(value: T) {
    println!("{}", value);
}

fn print_multiple<T>(value: T) where T: Display + Human, {
    println!("{}", value);
}

// 无法编译，rust 中不允许返回多个不同的类型
// fn get_human(value: bool) -> impl Human {
//     if value {
//         Teacher { name: "Lisa Teacher".to_string() }
//     } else {
//         Student { name: "Student".to_string() }
//     }
// }


// 泛型类
struct MyType<T> {
    x: T,
    y: T,
}

// 泛型方法
impl<T> MyType<T> {
    fn f(&self) -> &T {
        &self.x
    }
}

// 指定泛型的的类型
impl MyType<i32> {
    fn f2(&self) -> &i32 {
        &self.y
    }
}

// 普通的泛型方法
fn return_it<T>(v: &T) -> &T {
    &v
}

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn use_panic() {
    // 抛出异常
    panic!("crash here!");
}

enum MyEnum {
    TYPE1(i32),
    TYPE2(String),
    TYPE3,
}

enum Person {
    MAN(String),
    WOMAN(String),
}


struct Woman {
    name: String,
}

struct Man {
    name: String,
}

impl MyLocation {
    fn show_location(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    fn create_same(size: i32) -> Self {
        return Self {
            x: size,
            y: size,
            name: String::from("same size"),
        };
    }
}

#[derive(Debug)] // 这里必须标记才能打印
struct MyLocation {
    x: i32,
    y: i32,
    name: String,
}

struct MyUnitLike;

struct MyTuple1(i32, f32, String);

struct MyTuple2(i32, String, f32);

struct User {
    id: u32,
    name: String,
    password: String,
    email_address: String,
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn change(some_string: &mut String) {
    // some_string 不可变，尽管 s 是可变的，因为 some_string 是引用
    some_string.push_str(", world");
}

fn borrow_value(value: String) {
    println!("{}", value);
}


fn test_return() -> i32 {
    10 // 报错，rust 是表达式语言，最后一个值不能加上；分号
}

fn another_function() {
    // 另外一个函数，函数命名遵循使用下划线命名方法
    println!("Call another function!!")
}

fn print_params(x: i32, y: f64) {
    println!("x {x} y:{y}");
}
