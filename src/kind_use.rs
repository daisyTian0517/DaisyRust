use std::fs::File;  
use std::io::{self, ErrorKind};  

pub fn kind_base_use(){
    let result = File::open("non_existent_file.txt");  

    match result {  
        Ok(_) => println!("File opened successfully!"),  
        Err(e) => match e.kind() {  
            ErrorKind::NotFound => println!("File not found!"),  
            ErrorKind::PermissionDenied => println!("Permission denied!"),  
            _ => println!("An unexpected error occurred: {}", e),  
        },  
    }  
}

pub fn kind_costume_use(){
    match do_something(-1) {  
        Ok(_) => println!("Success"),  
        Err(e) => println!("Error occurred: {:?}, Kind: {}", e, e.kind()), 
    }  
}
#[derive(Debug)]  
pub enum MyError {  
    NotFound,  
    PermissionDenied,  
    Other(String),  
}  

impl MyError {  
    pub fn kind(&self) -> &'static str {  
        match self {  
            MyError::NotFound => "NotFound",  
            MyError::PermissionDenied => "PermissionDenied",  
            MyError::Other(_) => "Other",  
        }  
    }  
}  

fn do_something(value: i32) -> Result<(), MyError> {  
    if value < 0 {  
        return Err(MyError::NotFound);  
    }  
    if value > 10 {  
        return Err(MyError::PermissionDenied);  
    }  
    let other_error = String::from("This is other error");
    if value <=10 && value>=0{
        return Err(MyError::Other(other_error));
    }
    Ok(())  
}  