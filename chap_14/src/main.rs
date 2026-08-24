// use chap_14::kinds::PrimaryColor;
// Above is what's needed without pub use - see lib.rs
use chap_14::PrimaryColor;
fn main() {
    let color = PrimaryColor::Red;
    println!("Color is {color:?}");
}
