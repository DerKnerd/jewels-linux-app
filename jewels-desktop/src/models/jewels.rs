use crate::api;
use crate::api::device::get_devices;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QModelIndex, QString, QVariant};
use libjewels::collector::send_device_data;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!(<QAbstractListModel>);
        type QAbstractListModel;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
    }

    #[qenum(Jewels)]
    enum JewelsRoles {
        DeviceType,
        Model,
        Manufacturer,
        Storage,
        Ram,
        Cpu,
        Os,
    }

    impl cxx_qt::Threading for Jewels {}

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, device_type, cxx_name = "deviceType")]
        #[qproperty(QString, model)]
        #[qproperty(QString, manufacturer)]
        #[qproperty(f64, storage)]
        #[qproperty(f64, ram)]
        #[qproperty(QString, cpu)]
        #[qproperty(QString, os)]
        type Device = super::DeviceStruct;
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        #[qproperty(bool, is_loading)]
        #[qproperty(bool, loading_failed)]
        type Jewels = super::JewelsStruct;

        #[cxx_override]
        fn rowCount(&self, parent: &QModelIndex) -> i32;

        #[cxx_override]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        fn roleNames(&self) -> QHash_i32_QByteArray;

        #[inherit]
        fn beginResetModel(self: Pin<&mut Self>);

        #[inherit]
        fn endResetModel(self: Pin<&mut Self>);

        #[qinvokable]
        fn sendData(&self);

        #[qinvokable]
        fn checkEolDevices(&self);

        #[qinvokable]
        fn loadDevices(self: Pin<&mut Self>);
    }
}

#[derive(Default)]
pub struct DeviceStruct {
    device_type: QString,
    model: QString,
    manufacturer: QString,
    storage: f64,
    ram: f64,
    cpu: QString,
    os: QString,
}

impl From<api::device::Device> for DeviceStruct {
    fn from(value: api::device::Device) -> Self {
        let os =
            if value.os.version.clone().is_some_and(|val| {
                val.to_lowercase() == "unknown" || val.to_lowercase() == "rolling"
            }) {
                value.os.name.clone()
            } else {
                format!("{} {}", value.os.name, value.os.version.unwrap_or_default())
            };

        Self {
            device_type: value.device_type.into(),
            model: value.model.into(),
            manufacturer: value.manufacturer.into(),
            storage: value.storage,
            ram: value.ram,
            cpu: value.cpu.model.into(),
            os: os.into(),
        }
    }
}

#[derive(Default)]
pub struct JewelsStruct {
    is_loading: bool,
    loading_failed: bool,
    devices: Vec<DeviceStruct>,
}

impl ffi::Jewels {
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.devices.len() as i32
    }

    fn role_names(&self) -> ffi::QHash_i32_QByteArray {
        let mut hash = ffi::QHash_i32_QByteArray::default();
        hash.insert(ffi::JewelsRoles::DeviceType.repr, "deviceType".into());
        hash.insert(ffi::JewelsRoles::Model.repr, "model".into());
        hash.insert(ffi::JewelsRoles::Manufacturer.repr, "manufacturer".into());
        hash.insert(ffi::JewelsRoles::Storage.repr, "storage".into());
        hash.insert(ffi::JewelsRoles::Ram.repr, "ram".into());
        hash.insert(ffi::JewelsRoles::Cpu.repr, "cpu".into());
        hash.insert(ffi::JewelsRoles::Os.repr, "os".into());
        hash
    }

    fn data(&self, index: &ffi::QModelIndex, role: i32) -> QVariant {
        let role = ffi::JewelsRoles { repr: role };

        if let Some(DeviceStruct {
            device_type,
            model,
            manufacturer,
            storage,
            ram,
            cpu,
            os,
        }) = self.devices.get(index.row() as usize)
        {
            match role {
                ffi::JewelsRoles::DeviceType => return device_type.into(),
                ffi::JewelsRoles::Model => return model.into(),
                ffi::JewelsRoles::Manufacturer => return manufacturer.into(),
                ffi::JewelsRoles::Storage => return storage.into(),
                ffi::JewelsRoles::Ram => return ram.into(),
                ffi::JewelsRoles::Cpu => return cpu.into(),
                ffi::JewelsRoles::Os => return os.into(),
                _ => {}
            }
        }
        QVariant::default()
    }

    fn send_data(&self) {
        tokio::spawn(async move { send_device_data().await });
    }

    fn check_eol_devices(&self) {
        tokio::spawn(async move {
            crate::eol::eol_check().await;
        });
    }

    fn load_devices(mut self: Pin<&mut Self>) {
        self.as_mut().set_is_loading(true);
        self.as_mut().set_loading_failed(false);

        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            let (loading_failed, devices) = if let Ok(devices) = get_devices().await {
                (false, devices)
            } else {
                (true, vec![])
            };
            qt_thread
                .queue(move |mut jewels| {
                    jewels.as_mut().set_is_loading(false);
                    jewels.as_mut().set_loading_failed(loading_failed);
                    jewels.as_mut().begin_reset_model();
                    jewels.as_mut().rust_mut().devices = devices
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<DeviceStruct>>();
                    jewels.as_mut().end_reset_model();
                })
                .unwrap();
        });
    }
}
