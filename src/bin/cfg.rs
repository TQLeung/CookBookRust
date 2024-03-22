#![allow(dead_code)]
#![allow(unused)]
fn main() {
    #[cfg(feature = "printer")]
    {
        println!("priter!");
        println!("hello features!");
    }

    #[cfg(not(feature = "printer"))]
    println!("cargo r --bin cfg --features=printer");

    println!("{}\n{}", !3 as u8, usize::MAX);
    assert_eq!(!0, usize::MAX);
}
