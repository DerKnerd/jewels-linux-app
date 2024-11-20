use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "cloud.ulbricht.jewels",
            rust_files: &["src/qt.rs"],
            qml_files: &["../qml/ui/main.qml", "../qml/ui/pages/jewels.qml"],
            ..Default::default()
        })
        .build();
}
