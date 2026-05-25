use cxx_qt_lib::QString;
use libjewels::alpm::{InstallablePackage, UpdatablePackage};
use libjewels::aur::AurPackage;

#[derive(Clone, Default)]
pub struct Package {
    pub name: QString,
    pub version: QString,
    pub description: QString,
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
