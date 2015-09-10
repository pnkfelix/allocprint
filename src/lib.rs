#![feature(allocator, no_std, libc, core_slice_ext)]

#![no_std]
#![allocator]

extern crate libc;

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values. In practice, the alignment is a
// constant at the call site and the branch will be optimized out.
#[cfg(all(any(target_arch = "arm",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86",
              target_arch = "x86_64",
              target_arch = "aarch64")))]
const MIN_ALIGN: usize = 16;

use core::cmp;
use core::ptr;

mod p {
    use libc;
    use core::ptr;
    extern {
        fn printf(fmt: *const u8, ...);
    }

    pub type U = usize;

    pub fn print0(fmt: &[u8]) {
        unsafe {
            printf(fmt.as_ptr());
            libc::fflush(ptr::null_mut());
        }
    }

    pub fn printu(fmt: &[u8], u: U) {
        unsafe {
            printf(fmt.as_ptr(), u);
            libc::fflush(ptr::null_mut());
        }
    }

    pub fn printuu(fmt: &[u8], u1: U, u2: U) {
        unsafe {
            printf(fmt.as_ptr(), u1, u2);
            libc::fflush(ptr::null_mut());
        }
    }
    pub fn printuuu(fmt: &[u8], u1: U, u2: U, u3: U) {
        unsafe {
            printf(fmt.as_ptr(), u1, u2, u3);
            libc::fflush(ptr::null_mut());
        }
    }
    pub fn printuuuu(fmt: &[u8], u1: U, u2: U, u3: U, u4: U) {
        unsafe {
            printf(fmt.as_ptr(), u1, u2, u3, u4);
            libc::fflush(ptr::null_mut());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rust_allocate(size: usize,
                                         align: usize) -> *mut u8 {
    extern {
        fn posix_memalign(memptr: *mut *mut libc::c_void,
                          align: libc::size_t,
                          size: libc::size_t) -> libc::c_int;
    }

    let ret = if align <= MIN_ALIGN {
        libc::malloc(size as libc::size_t) as *mut u8
    } else {
        let mut out = ptr::null_mut();
        let ret = posix_memalign(&mut out,
                                 align as libc::size_t,
                                 size as libc::size_t);
        if ret != 0 {
            ptr::null_mut()
        } else {
            out as *mut u8
        }
    };
    p::printuuu(b"__rust_allocate size: %lu align: %lu => %p\n\0",
                size, align, ret as usize);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn __rust_deallocate(ptr: *mut u8,
                                           old_size: usize,
                                           align: usize) {
    p::printuuu(b"__rust_deallocate %p %lu %lu\n\0",
                ptr as usize, old_size, align);
    libc::free(ptr as *mut libc::c_void)
}

#[no_mangle]
pub unsafe extern "C" fn __rust_reallocate(ptr: *mut u8,
                                           old_size: usize,
                                           size: usize,
                                           align: usize) -> *mut u8 {
    p::printuuuu(b"__rust_reallocate %p %lu %lu %lu \n\0",
                 ptr as usize, old_size, size, align);
    if align <= MIN_ALIGN {
        libc::realloc(ptr as *mut libc::c_void,
                      size as libc::size_t)
            as *mut u8
    } else {
        let new_ptr = __rust_allocate(size, align);
        ptr::copy(ptr, new_ptr, cmp::min(size, old_size));
        __rust_deallocate(ptr, old_size, align);
        new_ptr
    }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(ptr: *mut u8,
                                            old_size: usize,
                                            size: usize,
                                            align: usize) -> usize {
    p::print0(b"__rust_reallocate_inplace\n\0");
    old_size
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize,
                                     align: usize) -> usize {
    size
}
