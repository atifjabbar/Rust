fn main() {
    // let int_val = 10;
    // let float_val = 2.3;
    // let bool_val = true;
    // let char_val = 'p';

    // let values = (int_val,float_val,bool_val,char_val);
    // println!("{0} {1} {2} {3}", values.0,values.1,values.2,values.3);

    // println!("{:b}", values.0);
    
    // println!("{:?}",values);

    let mut days_of_week = ["Sat","Sun","Mon","Tues","Wed","Thurs","Fri"];
    // Change the short values into long values os Array
    days_of_week[0] = "Saturday";
    days_of_week[1] = "Sunday";
    days_of_week[2] = "Monday";
    days_of_week[3] = "Tuesday";
    days_of_week[4] = "Wednesday";
    days_of_week[5] = "Thursday";
    days_of_week[6] = "Friday";
    
    println!("{:?}",days_of_week);

    //creat tuple store joining date, day of joining{Using the Array}, date of birth
    let person_values_of_joining = ("21/09/2019",days_of_week[0],"12/03/1990");
    println!("{:?}",person_values_of_joining);
    
    
    
    //let person_values_of_joining = ("21/09/2019",days_of_week,"12/03/1990");
    //println!("{} {} {}",person_values_of_joining.0,person_values_of_joining.1[0],person_values_of_joining.2);

}