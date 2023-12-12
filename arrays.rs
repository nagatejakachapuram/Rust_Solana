fn main(){
    let arr: [u8;3] = [1,3,5];
    let other_arr: [u8;5] = [100;5];
    println!("index: {}, length: {} ",arr[0],other_arr.len());
    println!("{:?}",other_arr);
}