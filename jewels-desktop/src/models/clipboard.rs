use cxx_qt::CxxQtType;
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
        #[qml_singleton]
        type Clipboard = super::ClipboardStruct;

        #[qinvokable]
        #[cxx_name = "copy"]
        fn copy_to_clipboard(self: Pin<&mut Self>, text: QString);
    }
}

pub struct ClipboardStruct {
    arboard: Arboard,
}

impl Default for ClipboardStruct {
    fn default() -> Self {
        Self {
            arboard: Arboard::new().unwrap(),
        }
    }
}

impl ffi::Clipboard {
    fn copy_to_clipboard(mut self: Pin<&mut Self>, text: QString) {
        self.as_mut()
            .rust_mut()
            .arboard
            .set_text(text.to_string())
            .unwrap();
    }
}
