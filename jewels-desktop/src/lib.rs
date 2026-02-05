use cstr::cstr;
use qmetaobject::prelude::*;
use crate::models::jewels::Jewels;
use crate::models::login::Login;

pub mod qt;
pub mod models;
pub mod authentication;

qrc!(pages,
    "cloud/ulbricht/jewels" {
        "qml/ui/MainApp.qml",
        "qml/ui/MainPage.qml",
        "qml/ui/pages/JewelsPage.qml",
        "qml/ui/pages/LoginPage.qml",
        "qml/ui/pages/UpdatesPage.qml",
        "icons/jewels.svg"
    }
);

fn run_app() {
    qmetaobject::log::init_qt_to_rust();

    pages();

    qt::app::set_desktop_file("dev.imanuel.jewels");

    qml_register_type::<Jewels>(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Jewels"),
    );
    qml_register_type::<Login>(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Login"),
    );

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/cloud/ulbricht/jewels/qml/ui/MainApp.qml".into());

    engine.exec();
}

#[tokio::main]
pub async  fn jewels_desktop() -> std::io::Result<()> {
    env_logger::init();
    run_app();
    Ok(())
}
