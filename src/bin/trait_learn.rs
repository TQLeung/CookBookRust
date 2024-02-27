/// 1、同时实现了Clone Trait，derive(Copy)， 编译器会禁止编译
/// 2、实现了Clone, 并标记了Copy， 
///     deref * 会调用隐式调用clone() ，无Copy标记则需要手动调用clone()
///     赋值绑定如果也标记了Copy，编译期也会隐式调用clone()
/// 3、Drop Trait，当绑定的变量离开作用域，编译器会自动调用drop(),
///    【而当整个函数结束（调用栈结束），才会真正地释放分配的内存】
/// 4、Deref、DerefMut Trait 作用于智能指针，Deref、DerefMut 均触发于 *,
/// 5、Display 作用域println! 类似的方法
/// 
///
use std::{fmt::Display, ops::{Deref, DerefMut}};
#[derive(Debug)]
struct A(String);
impl Clone for A {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Drop for A {
    fn drop(&mut self) {
        todo!()
    }
}
impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invoke display:A({})", self.0)
    }
}
impl Deref for A {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        eprintln!("deref A");
        &self.0
    }
}
impl DerefMut for A {
    fn deref_mut(&mut self) -> &mut Self::Target {
        eprintln!("deref mut A");
        &mut self.0
    }
}
fn main() {
    let mut a = A("20".to_string());
    let p = &a as *const A;
    unsafe{
        let xx: A = (&*p).clone();
        println!("*ptr = {}", xx);
    }
    // let b = *(&a);
    *a = "greet".to_string();
    println!("end : {a} - {}", *a);
    // println!("{a}{b}")
}
