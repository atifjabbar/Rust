use std::io;
use std::io::Read;
use std::fs::{self, File};
fn main(){
    loop {

        println!("Kindly enter what do you want to do on file \n 1) c for create \n 2) q for quit \n 3) s for show");
        let mut flag = String::new();
        io::stdin().read_line(&mut flag);
        let mut flag = flag.trim();
        if flag == "c" {

            println!("Enter your file name \n ");
            let mut file_name = String::new();
            io::stdin().read_line(&mut file_name);
            let mut file_name = file_name.trim();
        
            println!("Enter file data \n");
            let mut file_data = String::new();
            io::stdin().read_line(&mut file_data);
            let mut file_data = file_data.trim();
        
            let location = format!("E:\\Rust\\test15\\{}",file_name);
            fs::write(location, file_data).expect("Unable to write to write file");     
        }
        else if flag == "q"{
            break;
        }
        else if flag == "s"{
            
            println!("Enter your file name \n ");
            let mut file_name = String::new();
            io::stdin().read_line(&mut file_name);
            let mut file_name = file_name.trim();

            let location = format!("E:\\Rust\\test15\\{}",file_name);
            println!("{} \n {}", file_name, read_username_from_file(location).unwrap());
            //second method to read file and to eleminate fn block
            //println!("{} \n {}", file_name, fs::read_to_string(location));
            }
        }
    }
fn read_username_from_file(path: String) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
