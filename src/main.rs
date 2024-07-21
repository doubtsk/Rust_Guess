use rand::Rng; //引入随机数库
use std::cmp::Ordering; //引入匹配库
use std::io; //标准io库

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //随机数是数字，编译器自动识别为数字型

    loop {
        //循环
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() //标准io库引用
            .read_line(&mut guess) //检测用户操作，并返回用户值和Result值
            .expect("Failed to read line"); //根据Result值抓取错误

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            //这段做的比较多，先给guess做了个遮罩，复用guess，这里就简称guess1，声明guess1是无符号的32位数字
            //后面的操作是用来过滤掉不是数字的操作，主要用trim函数来处理字符串为数字，match用来匹配
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } //上面这一段是用来比较用户猜的数字和随机数的大小，在相等时退出循环
        }
    }
}
