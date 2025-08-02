use cpp::cpp;

cpp! {{
    #include <QtGui/QGuiApplication>
    #include <QtGui/QIcon>
}}

fn set_qapplication_icon(icon_path: &str) {
    let icon_path = std::ffi::CString::new(icon_path).unwrap();
    let icon_path = icon_path.as_ptr();
    cpp!(unsafe [icon_path as "const char *"] {
        QGuiApplication::setWindowIcon(QIcon(icon_path));
    })
}

pub fn set_desktop_file(desktop_file: &str) {
    let desktop_file = std::ffi::CString::new(desktop_file).unwrap();
    let desktop_file = desktop_file.as_ptr();
    cpp!(unsafe [desktop_file as "const char *"] {
        QGuiApplication::setDesktopFileName(desktop_file);
    })
}