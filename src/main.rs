#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;




#[no_mangle]
pub extern "C" fn _start() -> ! {
 println!("change da world my final message goodbye \n aioan");
 panic!("panic message");
loop{}
}



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
