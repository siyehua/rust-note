fn main() {
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
