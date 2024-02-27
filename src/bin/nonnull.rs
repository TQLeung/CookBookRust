use std::ptr::NonNull;
use std::{
    alloc::{self, dealloc, Layout},
    fmt::Display,
    ptr,
};

struct A<T: Display>(T);
impl<T> Drop for A<T>
where
    T: Display,
{
    fn drop(&mut self) {
        #[cfg(feature = "printer")]
        println!("droping A({})", self.0);
    }
}

fn main() {
    // 新建NonNull，悬垂指针，单保证对齐。用于初始化
    let _ = NonNull::<A<String>>::dangling();
    unsafe {
        for _ in 0..99999999999usize {
            let (ptr, layout) = exec();
            #[cfg(feature = "printer")]
            println!("-----------------");
            // dealloc(ptr.as_ptr() as *mut _, layout);
        }
    }
}

unsafe fn exec() -> (NonNull<A<String>>, Layout) {
    let layout = Layout::array::<A<String>>(1).unwrap();
    let ptr = alloc::alloc(layout);
    let ptr = match NonNull::new(ptr as *mut A<String>) {
        Some(p) => p,
        None => alloc::handle_alloc_error(layout),
    };
    ptr::write(ptr.as_ptr(), A("Hello Unsafe!".to_string()));
    // println!("{:#?}", *ptr.as_ptr());
    let _ = ptr::read(ptr.as_ptr());
    (ptr, layout)
}
