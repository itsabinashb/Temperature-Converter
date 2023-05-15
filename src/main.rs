use std::io;
fn main() {
    let mut temp = String::new();
    println!("Enter the temparature in Farenheit:");

    io::stdin().read_line(&mut temp).expect("Not valid input");
    let temp: u32 = temp.trim().parse().expect("Not a number");
    let cel = get_temp_in_cel(temp);

    println!("Temparature in celcius is: {cel}");

}

fn get_temp_in_cel(x: u32) -> u32 {
        let y = x - 32;
        y*5/9
}