#![allow(unused, non_snake_case, dead_code)]
fn c_array<const N:usize>(s:&[u8])->[u8;N]{
    assert!(s.len()<=N);
    let mut t=[0;N];
    t[0..s.len()].copy_from_slice(s);
    t
}
fn main(){
    let t=c_array::<5>(b"asd");
    assert_eq!(&t,&[97,115,100,0,0]);
}