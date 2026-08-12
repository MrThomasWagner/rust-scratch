use garden::vegetables::Asparagus;

pub mod garden;

fn main(){
    let spear: Asparagus = Asparagus::default();
    
    println!("Hello from the garden!");
    println!("{:?}", spear);
    println!("{:?}", spear.length);
}
