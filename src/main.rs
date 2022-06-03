use rust_guile::*;
use std::os;

extern "C" fn hello_world() -> SCM {
    println!("Hello world!");
    unsafe {
	scm_from_int8(0)
    }
}



fn main() {
    let mut ___args = std::env::args().map(|mut arg| arg.as_mut_ptr() as *mut os::raw::c_char).collect::<Vec<*mut os::raw::c_char>>();
    let argc = ___args.len() as os::raw::c_int;
    let argv: *mut *mut os::raw::c_char = ___args.as_mut_ptr();
	    
    init_scm();
    
    register_void_function!(b"hello-world\0", hello_world);

    run_scm(argc, argv);
}

