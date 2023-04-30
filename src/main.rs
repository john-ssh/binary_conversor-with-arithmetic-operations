use std::io;

fn main() {

    // TO OBTAIN THE INPUT NUMBER
    let mut number = String::new();
    let mut number_e = String::new();
    let mut first_number = String::new();
    let mut second_number = String::new();


    println!("Enter a number to go binary:");
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");
    let number: u32 = number.trim().parse().expect("Invalid input"); 

    // PRINT THE NUMBER IN BINARY
    println!("The number {} in binary is: {:b}", number, number);


    // CAPTURE THE INPUT OF THE DECIMAL NUMBERS TO GO BINARY THEN PERFORM MATH OPERATIONS
    println!("Enter the first decimal number:");
    io::stdin().read_line(&mut first_number).expect("Failed to read input");
    let number = first_number.trim().parse::<u32>().expect("Invalid decimal number");
    
    println!("Enter the second decimal number:");
    io::stdin().read_line(&mut second_number).expect("Failed to read input");
    let number2 = second_number.trim().parse::<u32>().expect("Invalid decimal number");

    
    // CONVERT THE DECIMALS TO BINARY
    let binary = format!("{:b}", number);
    let binary2 = format!("{:b}", number2);


    // PERFORM TE FOUR OPERATIONS
    let result_add = number.checked_add(number2).expect("Overflow occurred");
    let binary_add = format!("{:b}", result_add);
    println!("The numbers {} + {} in binary is: {}", binary, binary2, binary_add);

    let result_sub = number.checked_sub(number2).expect("Underflow occurred");
    let binary_sub = format!("{:b}", result_sub);
    println!("The number {} - {} in binary is: {}", binary, binary2, binary_sub);
    
    let result_div = number.checked_div(number2).expect("Division by zero");
    let binary_div = format!("{:b}", result_div);
    
    println!("The number {} / {} in binary is: {}", binary, binary2, binary_div);
    let result_mul = number.checked_mul(number2).expect("Overflow occurred");
    let binary_mul = format!("{:b}", result_mul);
    println!("The number {} * {} in binary is: {}", binary, binary2, binary_mul);

   
 

    // GET THE INPUT FOR THE EEE-754 32 BITS NUMBER, CONVERTS IT AND PRINTS IT
   println!("Enter a number to go EEE-754 32 bits");
    io::stdin().read_line(&mut number_e)
        .expect("Failed to read line");
    let number_e: f32 = number_e.trim().parse().expect("Invalid input");

    let bits = number_e.to_bits();
    let sign = (bits >> 31) & 0b1;
    let exponent = ((bits >> 23) & 0b1111111) as i32;
    let significand = bits & 0b111_1111_1111_1111_1111_1111;

    println!("The binary representation of {} in IEEE-754 32-bit format is: {} {}", number_e, sign, format!("{:08b} {:023b}", exponent, significand));


}
