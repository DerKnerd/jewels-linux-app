use cpp::cpp;

cpp! {{
    #include <QtGui/QGuiApplication>
    #include <QtGui/QIcon>
}}

pub fn set_desktop_file(desktop_file: &str) {
    let desktop_file = std::ffi::CString::new(desktop_file).unwrap();
    let desktop_file = desktop_file.as_ptr();
    cpp!(unsafe [desktop_file as "const char *"] {
        QGuiApplication::setDesktopFileName(desktop_file);
    })
}