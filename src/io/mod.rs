pub mod foo {
    pub fn write_foo(msg: &str) {
        unsafe {
            libc::write(libc::STDOUT_FILENO, msg.as_bytes().as_ptr() as _, msg.len());
        }
    }
}
