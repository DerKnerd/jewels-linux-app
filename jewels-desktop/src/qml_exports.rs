// src/qml_exports.rs
// One list, used for both registration + file generation.
#[macro_export]
macro_rules! get_qml_exports {
    ($mac:ident) => {
        $mac! {
            module_uri: "cloud.ulbricht.jewels",
            major: 1,
            minor: 0,

            types: [
                (Jewels, "Jewels"),
                (Login, "Login"),
                (Updates, "Updates"),
                (OneTimePasswords, "OneTimePasswords"),
            ],

            singletons: [
                (Clipboard, "Clipboard"),
                (Otp, "Otp"),
                (Owners, "Owners"),
            ],
        }
    };
}

pub use get_qml_exports as qml_exports;
