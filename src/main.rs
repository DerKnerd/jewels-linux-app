use models::config::Config;
use models::jewels::Jewels;
use cstr::cstr;
use qmetaobject::prelude::*;
use qmetaobject::qml_register_singleton_instance;

pub mod collector;
pub mod models;
mod qt;

qrc!(pages,
    "cloud/ulbricht/jewels" {
        "qml/ui/main.qml",
        "qml/ui/pages/jewels.qml",
        "icons/jewels.svg"
    }
);

fn main() -> std::io::Result<()> {
    env_logger::init();
    qmetaobject::log::init_qt_to_rust();

    pages();

    qt::app::set_desktop_file("dev.imanuel.jewels");

    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Jewels"),
        Jewels::new(),
    );
    qml_register_singleton_instance(
        cstr!("cloud.ulbricht.jewels"),
        1,
        0,
        cstr!("Config"),
        Config::new(),
    );

    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/cloud/ulbricht/jewels/qml/ui/main.qml".into());

    engine.exec();
    Ok(())
}
