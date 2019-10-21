///////////////////// Task ==> make struct for accounts take tax_rate, owner, balance and deduct 
/// ///////////////// tax from the the total balance and print it

// fn main() {
// struct Account {
//     tax_rate: f64,
//     owner: String,
//     balance: f64,
// }

// let user1 = Account {
//     tax_rate: 0.1,
//     owner: String::from("Atif Jabbar"),
//     balance: 100000.0,
// };

//     println!("Owner name is: {}, Balance is: {}, and Tax Rate is: {}",user1.owner,user1.balance,user1.tax_rate);
//     let tax_deduct = user1.balance - (user1.balance*user1.tax_rate);  
//     println!("Your balance after tax deduction is: {}",tax_deduct)
// }

///////////////////////////////////////////////////////////////////////////////////////////

// struct tool_Tax {
//     name: String,
//     amount: f32,
// }

// fn main() {
//     let user_one = tool_Tax {
//         name:String::from("Atif Jabbar"),
//         amount:3000.0,
//     };
//     let person_one = cal_tax(&user_one);
//     println!("{:?}",cal_tax(&user_one));
//     println!("Name: {}, Amount: {}, Tax-Amount: {}",person_one.0,person_one.1,person_one.2);
// }// compound data types save in HEAP and call through refence
// fn cal_tax(tool_tax: &tool_Tax)->(String,f32,f32) {
//     let person_name = tool_tax.name.clone();
//     let tax_amount = tool_tax.amount * 0.16;
//     (person_name,tool_tax.amount,tax_amount)
// }

///////////////////////////////////////////////////////////////////////////////////////////////

// struct completeName {
//     first_name: String,
//     last_name: String,
// }

// fn main() {
//     let name = completeName {
//         first_name:String::from:("Atif"),
//         last_name:String::from("Jabbar"),
//     };
//     println!("{}",generate_complete_name(&name));
// }


// fn generate_complete_name(nameStruct: &completeName) -> String {
//     let mut complete_name = String::new();
//     complete_name.push_str(&nameStruct.first_name);
//     complete_name.push_str(&nameStruct.last_name);
//     complete_name
// }

//////////////////////////////////////////////////////////////////////////////////////////////
/// task 2 --> write a program that take input parameters : name,email,gender,gross salary,
/// and rent charges(percentage) .then print his net salary with his details.

use std::io;

struct salary {
    name: String,
    email: String,
    gender: String,
    gross_salary: f64,
    rent_charges: f64,
}

fn main() {

    let mut Name = String::new();
    let mut Email = String::new();
    let mut Gender = String::new();
    let mut Gross_Sal = String::new();
    let mut House_Rent_Per = String::new();
    println!("Enter the Name: ");
    io::stdin().read_line(&mut Name);
    println!("Enter the email: ");
    io::stdin().read_line(&mut Email);
    println!("Enter the Gender: ");
    io::stdin().read_line(&mut Gender);
    println!("Enter the Gross Salary: ");
    io::stdin().read_line(&mut Gross_Sal);
    println!("Enter the Rent Charges: ");
    io::stdin().read_line(&mut House_Rent_Per);
    
    let res_one:f64 = Gross_Sal.trim().parse().unwrap();
    let res_two:f64 = House_Rent_Per.trim().parse().unwrap();

    let user_one = salary {
        name:Name,
        email:Email,
        gender:Gender,
        gross_salary:res_one,
        rent_charges:res_two,
    };
    let person_one = net_salary(&user_one);
//    println!("{:?}",net_salary(&user_one));
    println!("Name: {}",person_one.0);
    println!("email: {}",person_one.1);
    println!("Gender: {}",person_one.2);
    println!("Gross_Salary: {}",person_one.3);
    println!("House_Rent_Percentage: {}",person_one.4);
    println!("Net_Salary: {}",person_one.5);
}

fn net_salary(net_sal: &salary)->(String,String,String,f64,f64,f64) {
    let person_name = net_sal.name.clone();
    let e_mail = net_sal.email.clone();
    let gend = net_sal.gender.clone();
    let net_salary_ = net_sal.gross_salary * net_sal.rent_charges;
    (person_name,e_mail,gend,net_sal.gross_salary,net_sal.rent_charges,net_salary_)
}

/////////////////////////////////////////////////////////////////////////////////////////////////
// Sir task done on class


// Us std::io;
// #[derive(Debug)]

// struct userData {
//     name: String,
//     email: String ,
//     gender: String,
//     gross_salary: f64,
//     rent_charges: f64,
//     net_salary: f64,
// }

// fn main()
// {
//     println!("Enter The Name");
//     let mut input_one = String::new();
//     io::stdin().read_line(&mut input_one);

//     println!("Enter The Email ");
//     let mut input_two = String::new();
//     io::stdin().read_line(&mut input_two);

//     println!("Enter The Gender ");
//     let mut input_three = String::new();
//     io::stdin().read_line(&mut input_three);
    
//     println!("Enter The Gross Salary ");
//     let mut input_four = String::new();
//     io::stdin().read_line(&mut input_four);
//     let x:f64 = input_four.trim().parse().unwrap();
    
//     println!("Enter The Rent Charges In Percentage");
//     let mut input_five = String::new();
//     io::stdin().read_line(&mut input_five);
//     let y:f64 = input_five.trim().parse().unwrap();

//     let user_one_report = generate_report(input_one,input_two,input_three,x,y);

//     println!("{:?}",user_one_report);

// }

// fn generate_report(name: String,email: String,gender: String,gross_salary: f64, rent_charges:f64)->userData
// {
//     userData{
//         name,
//         email,
//         gender,
//         gross_salary,
//         rent_charges,
//         net_salary:(gross_salary*rent_charges),
//     }
// }