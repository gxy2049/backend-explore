// 获取用户输入并将结果打印为输出
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("随机数：{secret_number}");

    loop {
        println!("输入你的数字");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("获取用户输入失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字：{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜中了");
                break;
            }
        }
    }
}
