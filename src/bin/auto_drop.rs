use std::marker::PhantomData;

struct B(i32);
impl Drop for B{
    fn drop(&mut self) {
        println!("dropping B {}", self.0);
    }
}
struct A<'a, T>{
    ptr:*const T,
    _maker:PhantomData<&'a T>,
}

fn main(){
    let b = B(1);
    exec(&b);
    println!("main end!");
}

fn exec(b:&B){
    let a = A{
        ptr: b as *const B,
        _maker: PhantomData,
    };
}