use std::thread;
use std::time::Duration;

fn time_consumption_work(samay: u8) -> u8 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(5)); // time duration set karne ke liye har ek call ke beech
    samay
}

fn generate_workout(samay: u8, random_number: u8) {
    if samay < 15 {
        println!(
            "Have to make {} golgappe!",
            time_consumption_work(samay)
        );
        println!(
            "Have to make {} samsoa!",
            time_consumption_work(samay)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to take healthy diet today!");
        } else {
            println!(
                " Have to make {} pakode",
                time_consumption_work(samay)
            );
        }
    }
}
use std::io;
fn main() {

    let mut num = String::new();
    println!("Enter a value :");
    io::stdin()
       .read_line(&mut num)
       .expect("failed to read line");

    let first_num: u8 = num.trim().parse().expect("Not a String");

    let mut num = String::new();
    println!("Enter a value :");
    io::stdin()
       .read_line(&mut num)
       .expect("failed to read line");

    let sec_num: u8 = num.trim().parse().expect("Not a String");

    let simulated_user_specified_value = first_num;
    let simulated_random_number = sec_num;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
