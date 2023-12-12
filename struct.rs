// creating a bird game with damage 
fn main(){
    let name: String = String::from("Bird");
    let bird = Bird {name,attack : 5 };
    bird.print_name();
    println!("{} {}",bird.can_fly(),bird.is_animal());
    }
    struct Bird{
        name: String, // defining the feild and variables
        attack: u64
    }
    impl Bird{
        fn print_name(&self){
            println!("{}",self.name);   // implementing the moethods, values are self declaredfffffffffff
        }
    }
    impl Animal for Bird{
        fn can_fly(&self) -> bool {
            true
        }
        fn is_animal(&self) -> bool  {
            true
        }
    }

    trait Animal{
        fn can_fly(&self)-> bool,
        fn is_animal(&self) -> bool
        {
            true    
        }
    }