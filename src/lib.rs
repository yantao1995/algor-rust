// 是 pub 外部才可以调用
pub mod a {

    pub mod b {
        //pub外面才能调用
        pub fn call(s: &str) {
            println!("call:{}", s)
        }

        pub fn call_super() {
            super::super::call_ab(); // 相当于文件系统的 ../ 两个 super 就是 ../../
        }

        //外部调用不到
        fn private_fn() {
            println!("私有函数");
        }
    }
}

use crate::a::b;

pub fn call_ab() {
    crate::a::b::call("绝对路径"); //绝对路径
    a::b::call("相对路径"); //相对路径
    b::call("call");
}

// pub 同时可以将枚举 和 struct字段也变成公共的

// 多级目录

mod front_of_house;

fn call_dir() {
    front_of_house::eat_at_restaurant();
}
