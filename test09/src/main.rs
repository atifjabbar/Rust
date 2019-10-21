// struct empDetails {
//     id:i32,
//     name:String,
//     email:String,
//     gross_salary:f32
// }

// impl empDetails {
//     fn createUser(fid:i32,fname:String,femail:String)->empDetails {
//         empDetails {
//             id:fid,
//             name:fname,
//             email:femail,
//             gross_salary:50000.0
//         }
//     }
//     fn minus_tax(&self)->f32 {
//         &self.gross_salary-(self.gross_salary*0.20)
//     }
// }


// fn main() {
//     let emp_one = empDetails::createUser(01, String::from("Atif"), String::from("abc@abc.com"));
//     println!("{:?}",emp_one);
//     println!("{}",emp_one.minus_tax());
// }


///////////////////////////////////////////////////////////////////////////////////////////////////
/// write a program that to make challan receipt of car/Bike
/// Name, NIC, Car/Bike Number, Challan type
/// 
// #[derive(Debug)]

// use std::io;

// struct chalanDetail {
//     name:String,
//     nic:String,
//     vehicle_number:String,
//     chalan_type:String
// }

// impl chalanDetail {
//     fn createChalan(name:String,nic:String,vehicle_number:String,chalan_type:String)->chalanDetail {
//         chalanDetail {
//             name,
//             nic,
//             vehicle_number,
//             chalan_type
//         }
//     }

//     fn chalan_cond(&self)->i32 {
//         if &self.chalan_type == "car" {
//             500
//         }
//         else {
//             200
//         }
//     }
// }


// fn main() {
//     let chalan_one = chalanDetail::createChalan(String::from("Atif"),String::from("42201-4563722-5"),String::from("ABC-543"),String::from("car"));
//     let chalan_two = chalanDetail::createChalan(String::from("Ali"),String::from("12345-4563722-5"),String::from("KHI-999"),String::from("bike"));

//     println!("{:?}",chalan_one);
//     println!("{}",chalan_one.chalan_cond());

//     println!("{:?}",chalan_two);
//     println!("{}",chalan_two.chalan_cond());
// }


////////////////////////////////////////////////////////////////////////////////////////////////



// use std::io;
// #[derive(Debug)]
// struct chalanDetail {
//     name:String,
//     nic:String,
//     vehicle_number:String,
//     chalan_type:String
// }

// impl chalanDetail {
//     fn createChalan(name:String,nic:String,vehicle_number:String,chalan_type:String)->chalanDetail {
//         chalanDetail {
//             name,
//             nic,
//             vehicle_number,
//             chalan_type
//         }
//     }

//     fn chalan_cond(&self)->i32 {
//         if &self.chalan_type == "car" {
//             500
//         }
//         else if &self.chalan_type == "bike" {
//             200
//         }
//         else {
//             0
//         }
//     }
// }


// fn main() {

    
//     let mut Name = String::new();
//     let mut NIC = String::new();
//     let mut Veh_Num = String::new();
//     let mut Veh_Type = String::new();
    
//     println!("Enter the Name: ");
//     io::stdin().read_line(&mut Name);
//     println!("Enter the NIC: ");
//     io::stdin().read_line(&mut NIC);
//     println!("Enter the Vehicle Number: ");
//     io::stdin().read_line(&mut Veh_Num);
//     println!("Enter the Vehicle Type: ");
//     io::stdin().read_line(&mut Veh_Type);
    
//     let chalan_one = chalanDetail::createChalan(Name.trim().to_string(),NIC.trim().to_string(),Veh_Num.trim().to_string(),Veh_Type.trim().to_string());

//     println!("{:#?}",chalan_one);
//     println!("{}",chalan_one.chalan_cond());
// }

////////////////////////////////Traits//////////////////////////////////////////////////

use std::io;
#[derive(Debug)]
struct chalanDetail {
    name:String,
    nic:String,
    vehicle_number:String,
    chalan_type:String
}

pub trait makeChalan {
    fn createChalan(&self)->i32 {
        100
    }
}

impl chalanDetail {
    fn createChalan(name:String,nic:String,vehicle_number:String,chalan_type:String)->chalanDetail {
        chalanDetail {
            name,
            nic,
            vehicle_number,
            chalan_type
        }
    }
}

impl makeChalan for chalanDetail {
    fn chalan_cond(&self)->i32 {
        if &self.chalan_type == "car" {
            500
        }
        else if &self.chalan_type == "bike" {
            200
        }
        else {
            0
        }
    }
}

fn main() {

    
    let mut Name = String::new();
    let mut NIC = String::new();
    let mut Veh_Num = String::new();
    let mut Veh_Type = String::new();
    
    println!("Enter the Name: ");
    io::stdin().read_line(&mut Name);
    println!("Enter the NIC: ");
    io::stdin().read_line(&mut NIC);
    println!("Enter the Vehicle Number: ");
    io::stdin().read_line(&mut Veh_Num);
    println!("Enter the Vehicle Type: ");
    io::stdin().read_line(&mut Veh_Type);
    
    let chalan_one = chalanDetail::createChalan(Name.trim().to_string(),NIC.trim().to_string(),Veh_Num.trim().to_string(),Veh_Type.trim().to_string());

    println!("{:#?}",chalan_one);
    println!("{}",chalan_one.chalan_cond());
}