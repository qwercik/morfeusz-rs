use std::str;
use std::ffi::CStr;

pub fn i8_ptr_to_str<'a>(ptr: *const i8) -> &'a str {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    str::from_utf8(cstr.to_bytes()).unwrap()
}

pub fn i8_ptr_to_string(ptr: *const i8) -> String {
    let s = i8_ptr_to_str(ptr);
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_bytes_to_str() {
        let bytes = [65, 108, 97, 32, 109, 97, 32, 107, 111, 116, 97, 0];
        let ptr = bytes.as_ptr();
        let s = i8_ptr_to_str(ptr);

        assert_eq!(s, "Ala ma kota");
    }

    #[test]
    fn dummy_bytes_to_string() {
        let bytes = [65, 108, 97, 32, 109, 97, 32, 107, 111, 116, 97, 0];
        let ptr = bytes.as_ptr();
        let s = i8_ptr_to_string(ptr);

        assert_eq!(s, "Ala ma kota");
    }
}
