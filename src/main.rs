use algor_rust::a::b;
use rand::Rng;
use std::cmp::Ordering;
use std::{io, isize};
fn main() {
    //guess_number();

    //knowledge_normal_type();

    //println!(" knowledge_normal_fn(0): {}", knowledge_normal_fn(0));
    //println!(" knowledge_normal_fn(1): {}", knowledge_normal_fn(1));

    // let mut str = String::from("s1");
    // knowledge_owner(&mut str);
    // println!("str:{}", str);

    // let str_lice = &String::from("news word");
    // println!("slice_index:{}", &str_lice[0..knowledge_slice(&str_lice)]);

    // knowledge_struct();

    //knowledge_enum()

    algor_rust::call_ab();
    algor_rust::a::b::call("ssssss");
    algor_rust::a::b::call_super();

    b::call_super();
}

#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

impl IpAddressKind {
    fn call(&self) {}
}

fn value_in_addr(ip_addr: IpAddressKind) -> i32 {
    match ip_addr {
        //必须穷举所有可能性
        IpAddressKind::V4 => 4,
        IpAddressKind::V6 => {
            println!("println");
            6
        }
    }
}

fn knowledge_enum() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    four.call();
    six.call();

    let some_number = Some(5);
    let some_string = Some("asdsad");

    let absent_number: Option<i32> = None;

    // let sum = some_number + absent_number; //不能直接相加  得将 absent_number 转成 i32
    //println!("sum:{}", sum);

    fn mch(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = mch(five);

    let i = 8;
    match i {
        9 => {
            println!("---");
        }
        //二选一
        other => {
            println!("other:{}", other);
        }
        _ => (), //二选一
    }

    if let Some(max) = five {
        println!("if let some:{}", max)
    }
}

#[derive(Debug)]
struct User {
    age: i32,
    name: String,
}

impl User {
    //无法使用
    fn speak1() -> String {
        String::from("阿萨德")
    }
    //借用
    fn speak2(self) -> String {
        String::from("阿萨德")
    }
    //引用
    fn speak3(&self) -> String {
        String::from("阿萨德")
    }
}

//结构体
fn knowledge_struct() {
    let user1 = User {
        age: 11,
        name: String::new(),
    };
    // user1.name = String::from("value");
    println!("user.name:{}", user1.name);

    let mut user2 = User {
        age: 11,
        name: String::from("123"),
    };
    user2.age = 12;
    user2.name = String::from("value");
    println!("user2.age:{},name:{}", user2.age, user2.name);

    //调用
    let user3 = knowledge_struct_new_user(123);
    println!("user3.age:{},name:{}", user3.age, user3.name);

    //调用
    let user4 = User { age: 4, ..user3 };
    println!("user4.age:{},name:{}", user4.age, user4.name);

    //无字段元组
    struct Point(i32, i32, String);
    let p1 = Point(11, 22, String::from("point"));
    println!("point,0:{},1:{},2:{}", p1.0, p1.1, p1.2);

    //元组传参
    let tp = (3, 2);
    let tpr = knowledge_struct_tup(tp);
    println!("knowledge_struct_tup:{}", tpr);

    //结构体传参
    let user5: User = User {
        age: 12,
        name: String::new(),
    };
    let us5 = knowledge_struct_struct(&user5);
    println!("knowledge_struct_struct:{}", us5);
    println!("knowledge_struct_struct_done:{}", user5.age);
    println!("knowledge_struct_struct_println:{user5:?}");
    println!("knowledge_struct_struct_println:{user5:#?}");
    dbg!(user5);

    //方法
    // let spk = user4.speak1(); //无法执行
    // println!("user4 speak1:{}", spk);
    let spk = user4.speak3(); //
    println!("user4 speak2:{}", spk);
    let spk = user4.speak2(); //执行借用的话，user4就无了
    println!("user4 speak2:{}", spk);

    //元组 struct
    let t1: tp1 = tp1(1, 1, 0.123);
    println!("元组结构体t1:{}", t1.0);
}

struct tp1(i32, u32, f64);
struct tp2(i32, u32, f64); //与tp1不是同一个类型

fn knowledge_struct_struct(user: &User) -> i32 {
    user.age * 20
}

fn knowledge_struct_tup(dimens: (u32, u32)) -> u32 {
    dimens.0 * dimens.1
}

fn knowledge_struct_new_user(age: i32) -> User {
    User {
        age,
        name: String::from("knowledge_struct_new_user"),
    }
}

//切片
fn knowledge_slice(str: &str) -> usize {
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
    for i in 0..=3 {
        println!("a[{i}]:{}", a[i]);
    }
    for i in (1..2).rev() {
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

    let b = "   ";
    println!("let mut b {b}");
    let b = b.len();
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
