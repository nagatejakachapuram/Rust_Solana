fn main(){
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(6);
    let c: MyEnum= MyEnum::C{x: 10, y:12};
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);

}
#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C{x: i32, y:i32}
}