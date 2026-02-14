use qmetaobject::{qt_base_class, qt_method, QObject, QString};
use arboard::Clipboard as Arboard;

#[allow(non_snake_case)]
#[derive(QObject)]
pub struct Clipboard {
    base: qt_base_class!(trait QObject),

    pub copy: qt_method!(fn copy(&mut self, text: QString) {
        self.copy_to_clipboard(text.to_string());
    }),

    arboard: Arboard
}

impl Default for Clipboard {
    fn default() -> Self {
        Self {
            arboard: Arboard::new().unwrap(),
            base: Default::default(),
            copy: Default::default(),
        }
    }
}

impl Clipboard {
    fn copy_to_clipboard(&mut self, text: String) {
        self.arboard.set_text(text).unwrap();
    }
}
