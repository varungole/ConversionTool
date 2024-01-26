use std::io;

fn main()
{
    println!("Welcome to my Unit Converter");

    println!("Select the conversion you want");

    println!("USD TO INR");
    println!("Degree to Fahrenheit");
    println!("Kg to LBS");
    
    let mut select = String::new();

    io::stdin()
        .read_line(&mut select)
        .expect("Please select the appropriate options");

    let choice: i32 = select.trim().parse().unwrap(); // .trim() method requires when taking input


    match choice {
        1 => {
            println!("You selected USD to INR");
            println!("Enter the Dollar amount");
            let mut first = String::new();
            io::stdin().read_line(&mut first).expect("Please enter the Value in US Dollar");
            let input1: i32 = first.trim().parse().unwrap();

            let indian_rupee = input1 * 83;

            println!("The value in India Rupee is {}" , indian_rupee);
        },
        2=> {
            println!("You selected Degree to Fahrenheit");
            println!("Enter the Degrees");
            let mut first = String::new();
            io::stdin().read_line(&mut first).expect("Please enter the Value in Degree");
            let input1: i32 = first.trim().parse().unwrap();

            let fahrenheit = input1 * (9/5) + 32;
           

            println!("The value in Fahrenheit is {}" , fahrenheit);
        },
        3=> {
            println!("You selected KG to LBS");
            println!("Enter the Kg Weight");
            let mut first = String::new();
            io::stdin().read_line(&mut first).expect("Please enter the Value in KG");
            let input1: f32 = first.trim().parse().unwrap();

            let lbs = input1 * 2.205;


            println!("The value in LBS is {}" , lbs);
        }
        _ => {
            println!("Invalid choice!");
        }
    }

}