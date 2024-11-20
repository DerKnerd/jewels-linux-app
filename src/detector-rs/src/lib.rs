use cxx_qt_lib::QGuiApplication;

pub mod collector;
pub mod native;
pub mod qt;

fn run() -> i32 {
    QGuiApplication::new().exec()
}