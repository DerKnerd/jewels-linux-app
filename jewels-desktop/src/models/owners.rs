use crate::api;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QModelIndex, QString, QVariant};

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

    #[qenum(Owners)]
    enum OwnersRoles {
        Name,
        Email,
        IsAdmin,
        ProfilePicture,
    }

    impl cxx_qt::Threading for Owners {}

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, name)]
        #[qproperty(QString, email)]
        #[qproperty(bool, is_admin)]
        #[qproperty(QString, profile_picture)]
        type Owner = super::OwnerStruct;
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[base = QAbstractListModel]
        type Owners = super::OwnersStruct;

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
        fn load(&self);
    }
}

#[derive(Default)]
pub struct OwnerStruct {
    name: QString,
    email: QString,
    is_admin: bool,
    profile_picture: QString,
}

impl From<api::owner::Owner> for OwnerStruct {
    fn from(value: api::owner::Owner) -> Self {
        Self {
            name: value.name.into(),
            email: value.email.into(),
            is_admin: value.is_admin,
            profile_picture: value.profile_picture.into(),
        }
    }
}

#[derive(Default)]
pub struct OwnersStruct {
    owners: Vec<OwnerStruct>,
}

impl ffi::Owners {
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.owners.len() as i32
    }

    fn role_names(&self) -> ffi::QHash_i32_QByteArray {
        let mut hash = ffi::QHash_i32_QByteArray::default();
        hash.insert(ffi::OwnersRoles::Name.repr, "name".into());
        hash.insert(ffi::OwnersRoles::Email.repr, "email".into());
        hash.insert(ffi::OwnersRoles::IsAdmin.repr, "isAdmin".into());
        hash.insert(
            ffi::OwnersRoles::ProfilePicture.repr,
            "profilePicture".into(),
        );
        hash
    }

    fn data(&self, index: &ffi::QModelIndex, role: i32) -> QVariant {
        let role = ffi::OwnersRoles { repr: role };

        if let Some(OwnerStruct {
            name,
            email,
            is_admin,
            profile_picture,
        }) = self.owners.get(index.row() as usize)
        {
            match role {
                ffi::OwnersRoles::Name => return name.into(),
                ffi::OwnersRoles::Email => return email.into(),
                ffi::OwnersRoles::IsAdmin => return is_admin.into(),
                ffi::OwnersRoles::ProfilePicture => return profile_picture.into(),
                _ => {}
            }
        }
        QVariant::default()
    }

    fn load(&self) {
        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            let data = api::owner::get_owners().await.unwrap_or_default();
            qt_thread
                .queue(move |mut owners| {
                    owners.as_mut().begin_reset_model();
                    owners.as_mut().rust_mut().owners = data
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<OwnerStruct>>();
                    owners.as_mut().end_reset_model();
                })
                .unwrap();
        });
    }
}
