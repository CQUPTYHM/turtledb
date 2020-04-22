mod store;
mod utils;
mod executer;

fn main() {
    let s = "abc";
    let mut chars = s.chars();
    for i in 0..3 {
        println!("{}", chars.nth(i).unwrap());
    }
    let b = true;
    
}
