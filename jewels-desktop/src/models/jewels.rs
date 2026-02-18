use crate::api;
use crate::api::device::get_devices;
use libjewels::collector::send_device_data;
use qmetaobject::prelude::*;
use qmetaobject::{SimpleListItem, SimpleListModel};
use std::cell::RefCell;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Device {
    base: qt_base_class!(trait QObject),

    id: qt_property!(QString; NOTIFY idChanged),
    deviceType: qt_property!(QString; NOTIFY deviceTypeChanged),
    model: qt_property!(QString; NOTIFY modelChanged),
    manufacturer: qt_property!(QString; NOTIFY manufacturerChanged),
    storage: qt_property!(f64; NOTIFY storageChanged),
    ram: qt_property!(f64; NOTIFY ramChanged),
    cpu: qt_property!(QString; NOTIFY cpuChanged),
    os: qt_property!(QString; NOTIFY osChanged),

    idChanged: qt_signal!(),
    deviceTypeChanged: qt_signal!(),
    modelChanged: qt_signal!(),
    manufacturerChanged: qt_signal!(),
    storageChanged: qt_signal!(),
    ramChanged: qt_signal!(),
    cpuChanged: qt_signal!(),
    osChanged: qt_signal!(),
}

impl SimpleListItem for Device {
    fn get(&self, role: i32) -> QVariant {
        match role {
            0 => self.deviceType.clone().into(),
            1 => self.model.clone().into(),
            2 => self.manufacturer.clone().into(),
            3 => self.storage.clone().into(),
            4 => self.ram.clone().into(),
            5 => self.cpu.clone().into(),
            6 => self.os.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn names() -> Vec<QByteArray> {
        vec![
            "deviceType".into(),
            "model".into(),
            "manufacturer".into(),
            "storage".into(),
            "ram".into(),
            "cpu".into(),
            "os".into(),
        ]
    }
}

impl From<api::device::Device> for Device {
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
            base: Default::default(),
            id: Default::default(),
            deviceType: value.device_type.into(),
            model: value.model.into(),
            manufacturer: value.manufacturer.into(),
            storage: value.storage.into(),
            ram: value.ram.into(),
            cpu: value.cpu.model.into(),
            os: os.into(),
            idChanged: Default::default(),
            deviceTypeChanged: Default::default(),
            modelChanged: Default::default(),
            manufacturerChanged: Default::default(),
            storageChanged: Default::default(),
            ramChanged: Default::default(),
            cpuChanged: Default::default(),
            osChanged: Default::default(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Jewels {
    base: qt_base_class!(trait QObject),

    isLoading: qt_property!(bool; NOTIFY isLoadingChanged),
    loadingFailed: qt_property!(bool; NOTIFY loadingFailedChanged),
    devices: qt_property!(RefCell<SimpleListModel<Device>>; CONST),

    isLoadingChanged: qt_signal!(),
    loadingFailedChanged: qt_signal!(),

    sendData: qt_method!(
        fn sendData(&self) {
            self.send_data();
        }
    ),
    checkEolDevices: qt_method!(
        fn checkEolDevices(&self) {
            self.check_eol_devices();
        }
    ),
    loadDevices: qt_method!(
        fn loadDevices(&mut self) {
            self.load_devices();
        }
    ),
}

impl Jewels {
    fn send_data(&self) {
        tokio::spawn(async move { send_device_data().await });
    }

    fn check_eol_devices(&self) {
        tokio::spawn(async move {
            crate::eol::eol_check().await;
        });
    }

    fn load_devices(&mut self) {
        self.isLoading = true;
        self.loadingFailed = false;
        self.isLoadingChanged();
        self.loadingFailedChanged();

        let qptr = QPointer::from(&*self);
        let set_devices = qmetaobject::queued_callback(
            move |(failed, devices): (bool, Vec<api::device::Device>)| {
                if let Some(this) = qptr.as_pinned() {
                    let mut jewels_ref = this.borrow_mut();
                    jewels_ref.isLoading = false;
                    jewels_ref.loadingFailed = failed;

                    jewels_ref.loadingFailedChanged();
                    jewels_ref.isLoadingChanged();

                    let mut my_devices = jewels_ref.devices.borrow_mut();
                    my_devices.reset_data(devices.into_iter().map(Into::into).collect());
                }
            },
        );

        tokio::spawn(async move {
            if let Ok(devices) = get_devices().await {
                set_devices((false, devices));
            } else {
                set_devices((true, vec![]));
            }
        });
    }
}
