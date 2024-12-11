use std::arch::x86_64;
use std::io::{self, Write}; // 导入 io 和 Write trait

mod base_grammar;
mod base_grammmar2;
mod file_io;
mod iters_uses;
mod kind_use;
mod life_time;
mod print_info;

fn main() {
    // base_grammar::base_use();
    // base_grammmar2::base_use2();
    // iters_uses::iters_egs();
    // kind_use::kind_base_use();
    // kind_use::kind_costume_use();
    //life_time::life_use();
    //print_info::print_point();
    //print_info::print_info();

    let read_buffer = file_io::read_txt_buf();
    match read_buffer {
        Ok(()) => println!("file content is success"),
        Err(err) => println!("Error is {}", err),
    }

    let read_line = file_io::read_txt_line();
    match read_line {
        Ok(()) => println!("file content is success"),
        Err(err) => println!("Error is {}", err),
    }

    file_io::read_txt_to_string();
}
