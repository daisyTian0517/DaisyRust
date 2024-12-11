pub fn life_use(){
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
        println!("& str: {} VS {}", s1, s2);
    }
    println!("{} is longer", r);

    let s:String;
    {
        let s1 = String::from("rust");
        let s2 = String::from("ecmascript");
        s = longer2(s1, s2);//不推荐，仅供测试学习
        // println!("String: {} VS {}", s1, s2);// 报错
    }
    println!("{} is longer", s);

    let i: i32;
    {
        let a: i32 = 5;
        let b: i32 = 9;
        i = bigger(a, b);//基本数据类型：整数/浮点数类型、布尔值、字符、固定大小数组和元组等 所有权不转移
        println!("i32: {} VS {}", a, b);
    }
    println!("{} is bigger", i);

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut my_str = String::new();
    for c in chars {
        // Insert a char at the end of string
        my_str.push(c);
        // Insert a string at the end of string
        my_str.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', '|'];
    let trimmed_str: &str = my_str.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

fn longer2(s1: String, s2: String) -> String {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
