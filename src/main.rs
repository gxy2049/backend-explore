// 获取用户输入并将结果打印为输出
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess_number_main() {
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

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("值是{x}");
    }
    println!("外面的值是{x}");

    // 元组
    let tup = (500, 6.1, 1);
    let (x, y, z) = tup;
    println!("值y是${y}");

    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5];

    // 函数
    fn five() -> i32 {
        5
    }
    let x = five();

    // 控制流
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // 循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    // for循环
    for number in (1..4).rev() {
        println!("数字是{number}")
    }
}
