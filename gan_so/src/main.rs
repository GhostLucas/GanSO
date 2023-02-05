
#![no_std]  //  Implies no standard lib inclusion
            //  as result, panic handler must be implemented.
#![no_main] //  Implies that "C runtime zero" (a.k.a. crt0)
            //  can't be reached in this program, so we need to 
            //  write our own entry point chain for the system.

// we're telling the compiler to pack "PanicInfo" here
use core::panic::PanicInfo; 

// defining panic function behavior for unrecoverable errors 
// (such as my decision of learn programming for money)
#[panic_handler]                    // this is an attribute btw.
fn panic(_info: &PanicInfo) -> ! {  // "we ain't returning bro"
    loop {} // yo, this seems almost as productive as me when panicking
}

#[no_mangle] // telling rust we fucking need '_start' as function name
// The 'extern "C"' means we're using C calling convention (subroutines).
// By the way, why am I doing it in Rust then?
pub extern "C" fn _start() -> ! {   // '_start' is the default entry for 
                                    // most systems.
    loop {}
}
