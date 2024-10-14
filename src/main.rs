use rand::Rng;
use std::cmp::Ordering;
use std::str::Bytes;
use std::{io, isize};

fn main() {
    //guess_number();

    //knowledge_normal_type();

    //println!(" knowledge_normal_fn(0): {}", knowledge_normal_fn(0));
    //println!(" knowledge_normal_fn(1): {}", knowledge_normal_fn(1));

    // let mut str = String::from("s1");
    // knowledge_owner(&mut str);
    // println!("str:{}", str);

    let str_lice = &String::from("news word");
    println!("slice_index:{}", &str_lice[0..knowledge_slice(&str_lice)]);
}
//函数
fn knowledge_slice(str: &String) -> usize {
    let bts = str.as_bytes();
    for (i, &item) in bts.iter().enumerate() {
        if bts[i] == b' ' {
            return i;
        }
    }
    bts.len()
}

//函数
fn knowledge_owner(s1: &mut String) {
    let str = String::from("hello");
    println!("str:{}", str);
    let str2 = str;
    //println!("str:{}", str);  //str 无法使用
    println!("str2:{}", str2);
    let str3 = str2.clone();
    println!("str3:{}", str3);

    s1.push_str(" ssss");

    let mut s = String::from("asd");
    let r1 = &s;
    let r2 = &s;
    println!("r1:{}", r1);
    println!("r2:{}", r2);
    let r3 = &mut s;
    println!("r3:{}", r3);
    let r4 = s;
    println!("r4:{}", r4);
    // let r5 = &mut s; //被借走了，就不能用了
    // println!("r5:{}", r5);
}

//函数
fn knowledge_normal_fn(mut x: i32) -> i32 {
    println!("x:{}", x);
    // let x:i32: =let y = 6;
    // println!("x:{x}");
    // println!("y:{y}");
    let y = 0;
    println!("y:{}", y);
    x = y;
    println!("x:{},y:{}", x, y);

    let a = [2, 2, 3, 4];
    for i in a.iter() {
        println!("{}", i);
    }
    for i in (0..=3) {
        println!("a[{i}]:{}", a[i]);
    }

    let mut str = String::from("hello");
    println!("str:{}", str);
    str.push_str(" ,world");
    println!("str:{}", str);

    return if x < 1 {
        while x < 90 {
            x += 1;
        }
        x + 10
    } else {
        loop {
            x += 1;
            if x == 100 {
                break x * 2;
            }
        }
    };
}

//类型
fn knowledge_normal_type() {
    const CT: i32 = 1;
    println!("{CT}");
    let a = "       ";
    println!("let a {a}");
    let a = a.len();
    println!("let a {a}");

    let mut b = "   ";
    println!("let mut b {b}");
    let mut b = b.len();
    // b = b.len();             //errror
    println!("let mut b {b}");

    let mut u8: u8 = 255;
    u8 += 1;
    println!("u8:{u8}");

    let mut i8: i8 = i8::MAX;
    println!("i8:{i8}");
    i8 += 1;
    println!("i8:{i8}");

    let mut isize: isize = 1 << 63;
    isize -= 1;
    println!("isize:{isize}");
    isize += 1;
    println!("isize:{isize}");

    let tup: (i8, i32, u8) = (100, 100, 1);
    println!("tup0:{}", tup.0);
    println!("tup2:{}", tup.2);

    let sz = [1, 2, 3];
    println!("sz1:{}", sz[1]);
    let szm: [i32; 2] = [1, 2];
    println!("szm1:{}", szm[1]);
}

//猜数字
fn guess_number() {
    let rd = rand::thread_rng().gen_range(1, 101);
    println!("生成的数字是:{rd}",);
    let mut num = String::new();
    loop {
        println!("输入猜测的数字,1到100");
        num.clear();
        io::stdin().read_line(&mut num).expect("read_line err");
        println!("数字是: {num}");
        let num: i32 = match num.trim().parse() {
            Ok(abc) => abc,
            // Err(_) => 3,
            Err(error) => {
                println!("输入err:{},输入的数字:{}", error, num);
                continue;
            }
        };
        println!("输入的数字:{}", num);
        match num.cmp(&rd) {
            Ordering::Less => {
                println!("less");
                println!("done less");
            }
            Ordering::Equal => {
                println!("equals");
                println!("done equals");
                break;
            }
            Ordering::Greater => {
                println!("greater");
                println!("done greater")
            }
        }
    }
}
