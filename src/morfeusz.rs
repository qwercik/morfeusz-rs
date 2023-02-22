use std::ffi::CString;
use std::sync::Mutex;
use crate::helpers::i8_ptr_to_string;

static mut MORFEUSZ_MUTEX: Mutex<()> = Mutex::new(());

#[repr(C)]
#[derive(Clone)]
struct _InterpMorf {
    p: i32,
    k: i32,
    forma: *const i8,
    haslo: *const i8,
    interp: *const i8
}

extern "C" {
    fn morfeusz_about() -> *const i8;
    fn morfeusz_analyse(text: *const i8) -> *const _InterpMorf;
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct InterpMorf {
    p: i32,
    k: i32,
    forma: String,
    haslo: String,
    interp: String
}

pub fn about() -> String {
    let ptr = unsafe { morfeusz_about() };
    i8_ptr_to_string(ptr)
}

pub fn analyse(text: &str) -> Vec<InterpMorf> {
    let cstring = CString::new(text).unwrap();
    let text_ptr = cstring.as_ptr();

    let _guard = unsafe { MORFEUSZ_MUTEX.lock().unwrap() };
    let ptr = unsafe { morfeusz_analyse(text_ptr) };
    (0..).map(move |i| unsafe { (*ptr.offset(i)).clone() })
        .take_while(|e| e.p != -1)
        .map(|e| InterpMorf {
            p: e.p,
            k: e.k,
            forma: i8_ptr_to_string(e.forma),
            haslo: i8_ptr_to_string(e.haslo),
            interp: i8_ptr_to_string(e.interp)
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_return_text() {
        let result = about();
        assert!(!result.is_empty());
    }

    #[test]
    fn example_analysis_length() {
        let interpretations: Vec<_> = analyse_example();
        assert_eq!(interpretations.len(), 8);
    }

    #[test]
    fn example_analysis_el_3() {
        let interpretations = analyse_example();
        let el = &interpretations[3];

        assert_eq!(el.p, 1);
        assert_eq!(el.k, 2);
        assert_eq!(el.forma, "ma");
        assert_eq!(el.haslo, "mój:A");
        assert_eq!(el.interp, "adj:sg:nom.voc:f:pos");
    }

    fn analyse_example() -> Vec<InterpMorf> {
        let text = "Ala ma kota";
        analyse(text)
    }
}
