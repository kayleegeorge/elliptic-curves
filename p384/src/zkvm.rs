extern "C" {
    fn syscall_secp384r1_add(p: *mut u32, q: *const u32);
}

extern "C" {
    fn syscall_secp384r1_double(p: *mut u32);
}

const NUM_WORDS: usize = 24;

/// Add two field elements.
pub fn add_assign(limbs: &mut [u32; NUM_WORDS], other: &[u32; NUM_WORDS]) {
    unsafe {
        syscall_secp384r1_add(limbs.as_mut_ptr(), other.as_ptr());
    }
}

/// Double a field element.
pub fn double(limbs: &mut [u32; NUM_WORDS]) {
    unsafe {
        syscall_secp384r1_double(limbs.as_mut_ptr());
    }
}
