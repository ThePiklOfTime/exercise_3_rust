fn main() {
   let mut x: i32 = 0;
   println!("By how much do you want to increment the number?");
    while  x < 32767{
        
        if x < 32767 {
            println!("Current: {}. Increment by: ", x);
        }
        let mut increment = String::new();
        std::io::stdin().read_line(&mut increment).expect("Failed to read line");
        let increment: i32 = increment.trim().parse().expect("Please type a number!");
        if increment < 0 {
            println!("The given value is lower than 0.");
            continue;
        }else if increment == 0 {
            println!("The given value is 0. Ending the program.");
            return;
        }
        x += increment;
        
        
    } 
    println!("Enough incrementations.");
}
