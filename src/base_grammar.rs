use core::{option::Option::Some, result::Result::Err};
/* 基础语法 */
pub fn base_grammar() {
    let mut b = 10; // 可变变量 mutable，未指定类型

    let a: u64 = 123; //如果没有声明类型，a 将自动被判断为有符号 32 位整型变量，这对于 a 的取值范围有很大的影响。

    let y: f64 = 3.14; // y是不可变量 64位浮点数 指定类型

    fn add(a: i32, b: i32) -> i32 {
        // ->i32 代表返回值， 如果为空就是（）
        a + b
    }

    let mut counter = 0; //无限循环，可以使用 break 退出循环。
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number); // 宏规则
        number -= 1;
    }

    for number in 1..4 {
        println!("{}!", number);
    }

    let squared_vec: Vec<i32> = (0..10).into_iter().map(|x| x * x).collect(); // vec.foreach(o=>o*o)

    let filtered_vec: Vec<i32> = (0..10).into_iter().filter(|&x| x % 2 == 0).collect(); //vec.where(p=>p)
                                                                                    /*  */
    //所有权借用
    let s = String::from("hello");
    let len = calculate_length(&s); // 借用
    println!("The length of '{}' is {}.", s, len);

    // 枚举类型使用
    let coin = Coin::Dime;
    value_in_cents(coin);

    // 错误处理 Result
    let a: i32 = 15;
    let b: i32 = 10;
    let result: Result<i32, String> = divide(a, b);
    match result {
        Ok(value) => println!("The divive of {}/{} is {}.", a, b, value),
        Err(err_message) => println!("Error: {}.", err_message),
    }
    // 错误处理 Option
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let idx: usize = 2;
    let optionrs = get_element(idx, &v);

    match optionrs {
        Some(value) => println!("The {} value in vec is { }", idx, value),
        None => println!("No value"),
    }

    if let Some(value) = optionrs {
        println!("The {} value in vec is { }", idx, value);
    } else {
        println!("No value");
    }

    let value = optionrs.map(|v| v * 2).unwrap_or(0); // 如果是 Some，计算双倍，如果是 None，返回 0
    println!("Processed Value: {}", value);

    // 生命周期
    let s1 = String::from("Hello");
    let s2 = String::from("DaisyTian");
    let r1 = longesta(&s1, &s2);
    let r2 = longest(&s1, &s2);
    println!("r1 is {},r2 is {}", r1, r2);

    // 迭代器
    let numbers = vec![1, 2, 3, 4, 5];

    // 使用 fold 计算总和
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum); // 输出: Sum: 15

    // take 取值
    let taken: Vec<_> = numbers.iter().take(3).collect();
    println!("Taken: {:?}", taken); // 输出: Taken: [1, 2, 3]

    // enumerate 索引
    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    //迭代器链
    let arr = [1, 2, 3, 4, 5];
    let mut iter = arr.into_iter().peekable();
    while let Some(val) = iter.next() {
        if val % 2 == 0 {
            continue;
        }
        println!("{}", val);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn get_element(index: usize, vec: &Vec<i32>) -> Option<i32> {
    if index < vec.len() {
        Some(vec[index])
    } else {
        None
    }
}

fn longesta<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}
