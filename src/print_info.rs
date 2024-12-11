use std::fmt;
use num_format::{Locale, ToFormattedString}; // 导入相关模块  

pub fn print_point() {
    let point = Point { x: 1, y: 2 };
    println!("Point coordinates: {}", point);
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn print_info(){
    //千位分隔符
 println!("{:?}", 100000000.to_formatted_string(&Locale::zh)); // 输出 "100,000,000"
 println!("{:#?}", 1000000); // 输出 1,000,000
 
  //{:?} 是格式化字符串中的一个占位符
 let tuple = (1, "a", true);
 println!("tuple: {:?}", tuple);//输出：tuple: (1, "a", true)
 
//1.填充与对齐
println!("{:0>10}", "abc"); // 输出 0000000abc
println!("{:*>8}", "abc"); // 输出 *****abc
println!("{:*<8}", "abc"); // 输出 abc*****
println!("{:*^8}", "abc"); // 输出 **abc***

//2.数字格式化
println!("{:5}", 10); // 输出："   10"
println!("{:>10}", "left"); // 输出 "      left"
println!("{:<10}", "right"); // 输出 "right    "
println!("{:^10}", "center"); // 输出 "  center  "



//3.+: 正数和负数前都显示符号。
 println!("{:+}", 42);  // "+42"
 println!("{:+}", -42); // "-42"
 //-: 只有负数前显示符号（默认行为）
 println!("{:-}", 42);  // "42"
 println!("{:-}", -42); // "-42"
 //（空格）: 正数前显示一个空格，负数前显示负号。
 println!("{: }", 42);  // " 42"
 println!("{: }", -42); // "-42"
 
 //4. #号（进制表示前缀）（可选）
println!("{:#b}", 10);  // "0b1010"
println!("{:#o}", 10);  // "0o12"
println!("{:#x}", 10);  // "0xa"
println!("{:#X}", 10);  // "0xA"

//5. 宽度
println!("{:10}", "width");  // 宽度: 定义最小字段宽度的数字。

//6. 精度
println!("{:.2}", 3.14159);  // "3.14"
println!("{:.5}", 3.14159);  // "3.14159"
println!("{:.3}", "Rustacean");  // "Rus"

//7.0类型
println!("{:b}", 12);  // "1100"
println!("{:o}", 12);  // "14"
println!("{:x}", 255);  // "ff"
println!("{:X}", 255);  // "FF"
println!("{:e}", 1234.567);  // "1.234567e3"
println!("{:p}", &12);  // "0x55d4ba8fcf70" (memory address)

// 举例
//*: 使用 * 作为填充字符。
//>: 输出右对齐。
//+: 数字前显示符号（正号或负号）。
//#: 对于十六进制数，输出时添加 0x 前缀。
//10: 总宽度为 10。
//.3: 精度指定为三位数。
//x: 输出为十六进制。
println!("{:*>+#10.3x}", 255);//输出："*****+0xff"
}