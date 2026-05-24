use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("cloud.ulbricht.jewels")
            .qml_file("qml/ui/MainApp.qml")
            .qml_file("qml/ui/MainPage.qml")
            .qml_file("qml/ui/pages/InstallPage.qml")
            .qml_file("qml/ui/pages/JewelsPage.qml")
            .qml_file("qml/ui/pages/LoginPage.qml")
            .qml_file("qml/ui/pages/TotpCard.qml")
            .qml_file("qml/ui/pages/TwoFactorPage.qml")
            .qml_file("qml/ui/pages/UpdatesPage.qml"),
    )
    .file("src/models/login.rs")
    .file("src/models/jewels.rs")
    .build();
}
