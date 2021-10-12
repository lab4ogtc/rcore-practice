use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(_location) = info.location() {
        error!(
            "Panicked at {}:{} {}",
            _location.file(),
            _location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
