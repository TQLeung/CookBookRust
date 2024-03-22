#![allow(dead_code)]
#![allow(unused)]
fn main(){
    let s = "s".to_string();
    // p(&s[..]);   //Error!
    p("aa");
}

fn p(a:&'static str){
    println!("{}", a);
}