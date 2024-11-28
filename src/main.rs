
fn main() {
    // 闭包语法：let closure_name = |参数列表| 表达式或语句块;
    let s = String::from("hello");
    //按引用获取
    let ref_use = || println!("ref to use '{}'", s);
    ref_use();
    println!("参数 a 是 {}", s);

    //强制按值获取，所有权转移
    let take_s = move || println!("take ownship from '{}'", s);
    take_s(); // 输出 "hello"
              //println!("{}", s); // 这行代码将会报错，因为 s 的所有权已经被转移给了闭包

    //Copy 类型（例如，基本整数/浮点数类型、布尔值、字符、固定大小数组和元组等），那么在闭包中使用它并不会导致所有权转移
    let para_a: i32 = 5;
    let take_use = move || println!("{}", para_a);
    take_use();
    println!("参数 a 是'{}'", para_a);

    //可变借用捕获
    let mut num: i32 = 5;
    let mut change_num = || num += 1;
    change_num();
    println!("num after closure = {}", num); // 输出: num after closure = 6

    //闭包作为参数
    let double_val = |x| x * 2;
    let result = apply_as_param(5, double_val);
    println!("Result: {}", result); // 输出: Result: 10

    //作为返回值【使用 impl Fn 返回闭包】
    let add_five = apply_as_return(5);
    println!("5 + 3 = {}", add_five(3)); // 输出: 5 + 3 = 8
}

fn apply_as_param<F>(val: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(val)
}

fn apply_as_return(x: i32) -> impl Fn(i32) ->i32 {
    move |y| x + y
}
