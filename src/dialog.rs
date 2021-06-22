
#[macro_export]
macro_rules! msgbox {
    ( $msg:expr, $title:expr ) => {
        
        use winapi::{
            shared::windef::HWND,
            um::winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK}
        };
        
        let win_str = |s: &str| -> Vec<u16> {
            s.encode_utf16().chain(std::iter::once(0)).collect()
        };

        unsafe {
            MessageBoxW(
                0 as HWND,
                win_str( $msg ).as_ptr(),
                win_str( $title ).as_ptr(),
                MB_ICONINFORMATION | MB_OK,
            );
        }
    };
}


pub fn info(msg: &str) {
    msgbox!(msg, "Information");
}

pub fn error(msg: &str) {
    msgbox!(msg, "Error");
}

pub fn critical(msg: &str) {
    msgbox!(msg, "Error");

    panic!("{}", msg);
}
