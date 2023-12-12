fn main(){
    println!("{}",is_even(22));
}
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num %2; // digit stores the value of number % 2 value
    digit == 0 // return values must not cotain colon 
}