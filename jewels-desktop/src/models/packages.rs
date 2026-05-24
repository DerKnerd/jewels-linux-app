use libjewels::alpm::{InstallablePackage, UpdatablePackage};
use libjewels::aur::AurPackage;
use qmetaobject::{QObject, SimpleListItem, qt_base_class, qt_property, qt_signal};
use qttypes::{QByteArray, QString, QVariant};

#[derive(Clone, Default)]
pub struct Package {
    pub name: QString,
    pub version: QString,
    pub description: QString,
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct DownloadStatus {
    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY nameChanged),
    percent: qt_property!(f64; NOTIFY percentChanged),
    total: qt_property!(f64; NOTIFY totalChanged),
    current: qt_property!(f64; NOTIFY currentChanged),

    nameChanged: qt_signal!(),
    percentChanged: qt_signal!(),
    totalChanged: qt_signal!(),
    currentChanged: qt_signal!(),
}

impl DownloadStatus {
    pub fn reset(&mut self) {
        self.name = QString::default();
        self.percent = 0f64;
        self.total = 0f64;
        self.current = 0f64;
        self.nameChanged();
        self.percentChanged();
        self.totalChanged();
        self.currentChanged();
    }
    
    pub fn name(&self) -> QString {
        self.name.clone()
    }
    
    pub fn percent(&self) -> f64 {
        self.percent
    }
    
    pub fn total(&self) -> f64 {
        self.total
    }
    
    pub fn current(&self) -> f64 {
        self.current
    }
    
    pub fn set_name(&mut self, name: QString) {
        self.name = name;
        self.nameChanged();
    }
    
    pub fn set_percent(&mut self, percent: f64) {
        self.percent = percent;
        self.percentChanged();
    }
    
    pub fn set_total(&mut self, total: f64) {
        self.total = total;
        self.totalChanged();
    }
    
    pub fn set_current(&mut self, current: f64) {
        self.current = current;
        self.currentChanged();
    }
}

impl From<InstallablePackage> for Package {
    fn from(installable: InstallablePackage) -> Self {
        Package {
            name: QString::from(installable.name),
            version: QString::from(installable.version),
            description: QString::from(installable.description),
        }
    }
}

impl From<UpdatablePackage> for Package {
    fn from(updatable: UpdatablePackage) -> Self {
        Package {
            name: QString::from(updatable.name),
            version: QString::from(updatable.new_version),
            description: QString::from(updatable.description),
        }
    }
}

impl From<AurPackage> for Package {
    fn from(aur: AurPackage) -> Self {
        Package {
            name: QString::from(aur.name),
            version: QString::from(aur.version),
            description: QString::from(aur.description),
        }
    }
}

impl SimpleListItem for Package {
    fn get(&self, role: i32) -> QVariant {
        match role {
            0 => self.name.clone().into(),
            1 => self.version.clone().into(),
            2 => self.description.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn names() -> Vec<QByteArray> {
        vec![
            QByteArray::from("name"),
            QByteArray::from("version"),
            QByteArray::from("description"),
        ]
    }
}
