use std::{
    alloc::{self, Layout},
    ptr, thread,
    time::Duration,
};
fn main() {
    // byte Kib Mib Gib
    let n = 1024 * 1024 * 1024 * 5;
    let layout = Layout::array::<u8>(n).unwrap();
    loop {
        let ptr = alloc_in(layout);
        thread::sleep(Duration::from_secs(10));
        alloc_out(ptr, layout);
    }
}

fn alloc_in(layout: Layout) -> *mut u8{
    let mut i = 0usize;
    let ptr = unsafe {
        let ptr = alloc::alloc(layout);
        loop {
            ptr::write(ptr.offset(i as isize), 1);
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

fn alloc_out(ptr: *mut u8, layout: Layout) {
    let mut i = 0usize;
    unsafe {
        loop {
            ptr::read(ptr.offset(i as isize));
            i += 1;
            if i > layout.size() {
                break;
            }
        }
        alloc::dealloc(ptr, layout);
    }
    thread::sleep(Duration::from_secs(5));
    println!("dealloc end!");
}
