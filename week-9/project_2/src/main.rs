use std::io;
use std::io::Write;
use std::fs;

fn main() {
    let mut file = fs::File::create("pau-smis.csv").expect("create was unsuccessful");
    let header = "student name , matric number ,department , level ";
    file.write_all(header.as_bytes()).expect("write was unsuccessful");
    println!("how many students do you want to enter?");


    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("could not read");
    let qty = qty.trim().parse().expect("invalid input");
    let mut vname:Vec<String> = Vec::new();
    let mut vmatric:Vec<String> = Vec::new();
    let mut vdept:Vec<String> = Vec::new();
    let mut vlevel:Vec<u16> = Vec::new();

    for k in 0..qty { // for loop lets you enter the student info as many times as you have selcted
        

        let mut name = String::new();
        println!("enter student {} name", k + 1 );
        io::stdin().read_line(&mut name).expect("could not read");
        let name = name.trim().to_string();
        vname.push(name);


        let mut matric_number = String::new();
        println!("enter matric number {}",k + 1 );
        io::stdin().read_line(&mut matric_number).expect("could not read");
        let matric_number = matric_number.trim().to_string();
        vmatric.push(matric_number);

        let mut dept =  String::new();
        println!("enter your department");
        io::stdin().read_line(&mut dept).expect("could not read");
        let dept = dept.trim().to_string();
        vdept.push(dept);


        let mut level = String::new();
        println!("enter your level");
        io::stdin().read_line(&mut level).expect("could not read");
        let level:u16 =level.trim().parse().expect("invalid input");
        vlevel.push(level);

        let row = format!("{} , {} , {} , {}", vname[k] , vmatric[k] , vdept[k] , vlevel[k]);
        file.write_all(row.as_bytes()).expect("could not write");


    }
    println!("data written to file");   

    
    
}
    

