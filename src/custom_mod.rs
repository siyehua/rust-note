pub struct MyView {
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
            aaa();
        }
        fn aaa(){
            super::super::ttt(); // 可以调用父 module ，父 module 的函数不需要声明成 pub 的
        }
    }
}

fn ttt(){
    println!("I am super function!");
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::custom_mod::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


