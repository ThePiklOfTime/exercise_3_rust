fn main() {
   let mut x: i32 = 0;
   println!("By how much do you want to increment the number?");
    while  x < 32767{
        
        if x < 32767 {
            println!("Current: {}. Increment by: ", x);
        }
        let mut increment = String::new();
        std::io::stdin().read_line(&mut increment).expect("Failed to read line");
        let hello_world: i16 = match increment.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        };
        if hello_world < 0 {
            println!("The given value is lower than 0.");
            continue;
        }else if hello_world == 0 {
            println!("The given value is 0. Ending the program.");
            break
        }
       
        x += hello_world as i32;
         if x > 32767 {
            println!("Enough incrementations.");
        }
        
        
    } 
    
}
