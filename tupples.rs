fn main(){
    let tupple: (u8,bool,f32) = (5,false,3.2);
    let tupple2 = (3,5);
    println!("first {}, second {}, third {}",tupple.0,tupple.1,tupple.2);
    println!("{:?}",tupple2);
}