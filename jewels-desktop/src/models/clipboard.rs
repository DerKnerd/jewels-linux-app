use arboard::Clipboard as Arboard;
use cxx_qt_lib::QString;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Clipboard = super::ClipboardStruct;

        #[qinvokable]
        #[cxx_name = "copyToClipboard"]
        fn copy_to_clipboard(self: Pin<&mut Self>, text: QString);
    }
}

#[derive(Default)]
pub struct ClipboardStruct {}

impl ffi::Clipboard {
    fn copy_to_clipboard(self: Pin<&mut Self>, text: QString) {
        Arboard::new().unwrap().set_text(text.to_string()).unwrap();
    }
}
