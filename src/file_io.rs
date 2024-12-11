use std::fs;
use std::io::{self, prelude::*, BufReader, Read};


pub fn read_txt_buf() -> io::Result<()> {
    //unwrap() 如果失败，会 panic
    let mut file = fs::File::open("C:\\Users\\daisy\\Desktop\\Test\\test.log").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    // 持续读取直到文件结束
    loop {
        // 定义一个缓冲区，存储每次读取的数据
        let mut buffer = [0u8; 1024]; // 每次读取 1024 字节
        let bytes_read = buf_reader.read(&mut buffer)?;

        // 如果读取到 0 字节，表示文件已读完
        if bytes_read == 0 {
            break; // 退出循环
        }

        // 将读取的字节转换为字符串并追加到 contents 中
        contents.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
    }

    // 打印文件的完整内容
    println!("File contents:\n{}", contents);

    Ok(())
}

use std::path::Path;
use std::fs::File; 
pub fn read_txt_line() -> io::Result<()>{
     // 指定文件路径  
     let path = Path::new("C:\\Users\\daisy\\Desktop\\Test\\test.log");  
     let file = File::open(&path)?;  
 
     // 创建一个 BufReader 以便按行读取  
     let reader = io::BufReader::new(file);  
 
     // 遍历每一行并打印  
     for line in reader.lines() {  
         // 处理每一行的结果  
         match line {  
             Ok(content) => println!("{}", content),  
             Err(e) => eprintln!("Error reading line: {}", e),  
         }  
     }  
 
     Ok(())  
}

pub fn read_txt_to_string(){
    let text = fs::read_to_string("C:\\Users\\daisy\\Desktop\\Test\\test.log").unwrap();
    println!("{}", text);
}

