fn main() {
    #[cfg(feature = "printer")]
    {
        println!("priter!");
        println!("hello features!");
    }

    #[cfg(not(feature = "printer"))]
    println!("cargo r --bin cfg --features=printer");
}
