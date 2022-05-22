use std::slice;

static mut COUNTER: u32 = 0;
fn main() {
    let mut v = vec![1,2,3,4,5,6,7,8];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6,7,8]);

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("{:?}", values);
    // static mut global variables are unsafe
    unsafe {
        COUNTER += 1;
    }

}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = values.len();
    assert!(mid <= len);
    let ptr = values.as_mut_ptr();


    // compile error because we borrow mut from values two times. 
    // (&mut values[..mid], &mut values[mid..])
    // SAFETY: Never borrow as mutable the same part of the vector.
    unsafe {
        (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len-mid),
    )
    }
}

unsafe trait Foo {

}

unsafe impl Foo for i32 {
    
}
