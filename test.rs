#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::str;

mod crlib;
use crlib::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let panic_msg = b"PANIC occurred!\n\0";
        write(2, panic_msg.as_ptr(), panic_msg.len() - 1);
        quit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    // Print welcome message
    printf(b"=== crlib Usage Demo ===\n\0".as_ptr() as PCChar);
    
    let current_dir = pwd();
    if !current_dir.is_null() {
        printf(b"Current directory: %s\n\0".as_ptr() as PCChar, current_dir);
        } else {
            printf(b"Current directory: <unknown>\n\0".as_ptr() as PCChar);
        }
        
        // Math operations demo
        printf(b"\n--- Math Operations ---\n\0".as_ptr() as PCChar);
        let angle = 1.5707963; // π/2
        printf(b"sin(pi/2) = %.6f\n\0".as_ptr() as PCChar, sin(angle));
        printf(b"cos(pi/2) = %.6f\n\0".as_ptr() as PCChar, cos(angle));
        printf(b"sqrt(16) = %.2f\n\0".as_ptr() as PCChar, sqrt(16.0));
        printf(b"pow(2, 8) = %.0f\n\0".as_ptr() as PCChar, pow(2.0, 8.0));
        
        // String manipulation demo
        printf(b"\n--- String Operations ---\n\0".as_ptr() as PCChar);
        let test_str = b"hello world\0";
        printf(b"Test string: %s\n\0".as_ptr() as PCChar, test_str.as_ptr() as PCChar);
        
        // Token splitting demo
        printf(b"\n--- Token Splitting ---\n\0".as_ptr() as PCChar);
        let input = b"rust programming language\0";
        let mut current = input.as_ptr();
        let mut token_count = 1;
        
        printf(b"Splitting: %s\n\0".as_ptr() as PCChar, current as PCChar);
        
    
        for _ in 0..10 {
            let token = splitft(current);
            if token.word.is_null() || *token.word == 0 {
                break;
            }
            printf(b"Token %d: %s\n\0".as_ptr() as PCChar, token_count, token.word as PCChar);
            current = token.rest;
            if current.is_null() || *current == 0 {
                break;
            }
            token_count += 1;
        }
        
        // Number printing demo
        printf(b"\n--- Number Printing ---\n\0".as_ptr() as PCChar);
        printf(b"Numbers: \0".as_ptr() as PCChar);
        for i in 0..10 {
            uprint(i);
            if i < 9 {
                print(b", \0".as_ptr() as PCChar);
            }
        }
        print(b"\n\0".as_ptr() as PCChar);
        
        // String comparison demo
        printf(b"\n--- String Comparison ---\n\0".as_ptr() as PCChar);
        let str1 = b"hello\0";
        let str2 = b"hello\0";
        let str3 = b"world\0";
        
        if matchpcchar(str1.as_ptr(), str2.as_ptr()) {
            printf(b"'hello' == 'hello' OK\n\0".as_ptr() as PCChar);
        }
        
        if !matchpcchar(str1.as_ptr(), str3.as_ptr()) {
            printf(b"'hello' != 'world' OK\n\0".as_ptr() as PCChar);
        }
        
        // Time demo
        printf(b"\n--- Time Operations ---\n\0".as_ptr() as PCChar);
        let current_time = time(core::ptr::null_mut());
        printf(b"Current timestamp: %ld\n\0".as_ptr() as PCChar, current_time);
        
        // Memory allocation demo
        printf(b"\n--- Memory Operations ---\n\0".as_ptr() as PCChar);
        let ptr = malloc(1024);
        if !ptr.is_null() {
            printf(b"Allocated 1024 bytes successfully\n\0".as_ptr() as PCChar);
            free(ptr);
            printf(b"Memory freed\n\0".as_ptr() as PCChar);
        } else {
            printf(b"Memory allocation failed\n\0".as_ptr() as PCChar);
        }
        
        printf(b"\n=== Demo Complete ===\n\0".as_ptr() as PCChar);
   
        quit(0);
}
