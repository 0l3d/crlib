# crlib
A simple, no-std Rust library providing basic C library bindings and utility functions for systems programming.

## Features
* **No Standard Library**: Works in `#![no_std]` environments
* **C Bindings**: Direct access to essential C library functions
* **Utility Functions**: Helper functions for common operations
* **Memory Safe Wrappers**: Safer interfaces for C functions where possible
* **Cross-Platform**: Works on systems with standard C library

## Included Functions

### I/O Operations
* `printf`, `fprintf`, `sprintf`, `snprintf` - Formatted output
* `scanf`, `fscanf`, `sscanf` - Formatted input
* `write`, `read` - Low-level I/O
* `print` - Simple string printing
* `getch` - Single character input
* `input` - Line input with buffer

### Math Functions
* `sin`, `cos`, `tan`, `atan`, `atan2` - Trigonometric functions
* `sqrt`, `pow`, `exp`, `log`, `log10` - Mathematical operations
* `floor`, `ceil`, `fabs` - Rounding and absolute value

### Memory Management
* `malloc`, `realloc`, `free` - Dynamic memory allocation

### File System
* `chdir`, `getcwd` - Directory operations
* `cd`, `pwd` - Wrapper functions

### String Operations
* `splitft` - Token splitting function
* `matchpcchar` - String comparison
* `ensure_null_terminated` - Safe string handling

### System Operations
* `system`, `exec` - Command execution
* `exit`, `quit` - Program termination
* `time`, `sleep`, `usleep` - Time operations

## Usage
Add this library to your `#![no_std]` Rust project:

```rust
#![no_std]
#![no_main]

mod crlib;
use crlib::*;

#[no_mangle]
pub extern "C" fn main() -> i32 {
    unsafe {
        printf(b"Hello, World!\n\0".as_ptr() as PCChar);
        
        // Math operations
        let result = sqrt(16.0);
        printf(b"sqrt(16) = %.2f\n\0".as_ptr() as PCChar, result);
        
        // String operations
        let str1 = b"hello\0";
        let str2 = b"world\0";
        if matchpcchar(str1.as_ptr(), str2.as_ptr()) {
            printf(b"Strings match!\n\0".as_ptr() as PCChar);
        }
        
        // Token splitting
        let input = b"rust is awesome\0";
        let token = splitft(input.as_ptr());
        printf(b"First token: %s\n\0".as_ptr() as PCChar, token.word as PCChar);
        
        0
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { quit(1); }
}
```

## Safety Notes
⚠️ **Important**: Most functions in this library are marked as `unsafe` because they directly interface with C library functions. Always ensure:
* Strings are null-terminated when required
* Buffer sizes are adequate
* Pointers are valid before dereferencing
* Memory allocated with `malloc` is freed with `free`

## Examples

### Basic Math Operations
```rust
unsafe {
    let angle = 1.5707963; // π/2
    printf(b"sin(π/2) = %.6f\n\0".as_ptr() as PCChar, sin(angle));
    printf(b"cos(π/2) = %.6f\n\0".as_ptr() as PCChar, cos(angle));
}
```

### String Manipulation
```rust
unsafe {
    let text = b"hello world\0";
    let token = splitft(text.as_ptr());
    printf(b"Word: %s\n\0".as_ptr() as PCChar, token.word as PCChar);
}
```

### Memory Management
```rust
unsafe {
    let ptr = malloc(1024);
    if !ptr.is_null() {
        // Use the memory
        free(ptr);
    }
}
```

## Contributing
Feel free to submit issues and pull requests to improve this library.

## License
This project is open source. Please check the repository for license details.

## Author
Created by 0l3d

*This library is designed for educational purposes and systems programming where direct C library access is needed in a no-std Rust environment.*
