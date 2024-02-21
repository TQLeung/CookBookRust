/// 计算机操作系统的惰性分配。其基本原理是，当一个进程请求分配一大块内存时，操作系统会在逻辑上满足这个请求，但实际上并不立即在物理内存中分配相应的空间。只有当进程实际访问这些内存时（例如，写入数据）
use std::{
    alloc::{self, Layout},
    ptr, thread,
    time::Duration,
};

#[derive(Debug)]
struct A(u8);
// trait N{
//     fn new()->
// }
impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping MyStruct!");
    }
}

fn main() {
    // byte Kib Mib Gib
    let n = 1024 * 1024 * 1024 * 5;
    // Layout 只是计算需要分配的内存，及其对齐方式？
    let layout = Layout::array::<A>(n).unwrap();
    loop {
        let ptr = alloc_in(layout);
        thread::sleep(Duration::from_secs(10));
        alloc_out(ptr, layout);
    }
}

fn alloc_in(layout: Layout) -> *mut A{
    let mut i = 0usize;
    let ptr = unsafe {
        let ptr = alloc::alloc(layout) as *mut A;
        loop {
            ptr::write(ptr.offset(i as isize), A(1));   
            i += 1;
            if i > layout.size() {
                break;
            }
        }
        ptr
    };
    thread::sleep(Duration::from_secs(5));
    println!("alloc end!");
    ptr
}

fn alloc_out(ptr: *mut A, layout: Layout) {
    let mut i = 0usize;
    unsafe {
        loop {
            // 读取后，当绑定出范围，将调用Drop Trait
            let _ = ptr::read(ptr.offset(i as isize));
            // i += 1;
            // if i > layout.size() {
                break;
            // }
        }
        alloc::dealloc(ptr as *mut u8, layout); // 函数执行完毕 才会真正地释放内存！
    }
    thread::sleep(Duration::from_secs(5));
    println!("dealloc end!");
}
