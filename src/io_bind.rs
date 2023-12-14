mod c { extern "C" {
    pub fn test() -> libc::c_int;
} }

pub fn test() -> i32 {
    (unsafe {c::test()}) as i32
}

