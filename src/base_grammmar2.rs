use std::thread;
pub fn base_use2(){
      // 闭包语法：let closure_name = |参数列表| 表达式或语句块;
      let s = String::from("hello");
      //按引用获取
      let ref_use = || println!("ref to use '{}'", s);
      ref_use();
      println!("参数 a 是 {}", s);
  
      //强制按值获取，所有权转移
      let take_s = move || println!("take ownship from '{}'", s);
      take_s();
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
  
      // 作为返回值【使用 Box<dyn Fn> 返回闭包】
      let add_ten = apply_as_box(10);
      println!("10 + 6 = {}", add_ten(10));
  
      // Fn: 不需要修改捕获的变量，闭包可以多次调用。
      let name = String::from("Rust");
      let print_name = move || println!("Hello, {}!", name); // 使用 move 强制捕获所有权
      call_closure(print_name);
      // println!("{}", name); // 若取消注释，将报错，name 的所有权已被移动
  
      // 处理错误
      let numbers = [-10, -5, 0, 3, 5, -2];
      let index = find_first_positive(&numbers, |x| x > 0);
      match index {
          Some(i) => println!("第一个正数的索引: {}", i),
          None => println!("没有找到正数"),
      }
  
      let even_nums = [-10, -5, 0, 3, 5, -2];
      let all_even = find_all_evens(&even_nums, |x| x % 2 == 0);
      if all_even == true {
          println!("集合中所有数都是偶数");
      } else {
          println!("集合中含有奇数");
      }
  
      // 多线程
      let handles = (1..100)
          .into_iter()
          .map(|num: i32| {
              thread::spawn(move || {
                  let result = num * num;
                  println!(
                      "the current thread id is {:?},the result is {}",
                      thread::current().id(),
                      result
                  );
              })
          })
          .collect::<Vec<_>>();
      for handle in handles {
          handle.join().unwrap(); //等待线程结束
      }
  
      //函数作为参数
      let square = |x| x * x;
      // 调用函数，并传入闭包作为参数，对数字进行平方运算
      let result = apply_operation(num, square);
      // 输出结果
      println!("Square of {} is {}", num, result);
}

// 定义一个函数，接受一个闭包作为参数，将闭包应用到给定的数字上
fn apply_operation<F>(num: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(num)
}

pub fn find_first_positive(nums: &[i32], is_positive: impl Fn(i32) -> bool) -> Option<usize> {
    nums.iter().position(|&x| is_positive(x))
}

fn find_all_evens(nums: &[i32], all_evens: impl Fn(i32) -> bool) -> bool {
    nums.iter().all(|&x| all_evens(x))
}

fn apply_as_param<F>(val: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(val)
}

fn apply_as_return(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn apply_as_box(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

fn call_closure<F>(f: F)
where
    //F: Fn(), // 后边 f()只调用多次
    // F:FnMut //需要修改捕获的变量，闭包可以多次调用。
    F: FnOnce(), // 后边 f()只调用一次
{
    f();
}
