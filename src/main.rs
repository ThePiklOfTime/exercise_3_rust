fn main() {
   let mut x: i32 = 0;
    while  x < 32767{
        println!("By how much do you want to increment the number?");
        if x < 32767 {
            println!("Current: {}. Increment by:", x);
        }
        let mut increment = String::new();
        std::io::stdin().read_line(&mut increment).expect("Failed to read line");
        let increment: i32 = increment.trim().parse().expect("Please type a number!");
        x += increment;
        
        
    } 
    println!("Enough incrementations.");
}
