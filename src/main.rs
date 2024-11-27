fn main() {
    // 闭包语法：let closure_name = |参数列表| 表达式或语句块;
    let s = String::from("hello");
    //按引用获取
    let ref_use = || println!("ref to use:{}", s);
    ref_use(); 
    println!("参数 a 是{}", s);

    //强制按值获取，所有权转移 
    let take_s = move || println!("take ownship from {}", s);
    take_s(); // 输出 "hello"
    //println!("{}", s); // 这行代码将会报错，因为 s 的所有权已经被转移给了闭包

    //Copy 类型（例如，基本整数/浮点数类型、布尔值、字符、固定大小数组和元组等），那么在闭包中使用它并不会导致所有权转移
    let para_a: i32 = 5;
    let take_use = move || println!("{}", para_a);
    take_use();
    println!("参数 a 是{}", para_a);//不报错，原因未知？

    //可变借用捕获
    let mut change_num = || num += 1;
    change_num();
    println!("num after closure = {}", num); // 输出: num after closure = 6
}
