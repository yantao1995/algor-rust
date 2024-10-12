use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //guess_number();
    knowledge();
}

fn knowledge() {
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
