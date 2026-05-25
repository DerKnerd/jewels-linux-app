use crate::api;
use crate::api::owner::get_owners;
use cxx_qt::CxxQtType;
use cxx_qt::Threading;
use cxx_qt_lib::{QModelIndex, QString, QStringList, QVariant};
use std::collections::BTreeSet;
use std::pin::Pin;

fn get_icon_source(
    brand_icon_similarity: f64,
    simple_icon_similarity: f64,
    brand_icon: String,
    simple_icon: String,
) -> QString {
    if brand_icon_similarity == 0f64 && simple_icon_similarity == 0f64 {
        "/static/img/default.svg".into()
    } else if brand_icon_similarity < simple_icon_similarity {
        format!("/api/icons/{simple_icon}").into()
    } else {
        format!("/api/icons/{brand_icon}").into()
    }
}

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

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "new_my_otp_list"]
        fn new_ptr() -> *mut MyOneTimePasswordList;

        #[rust_name = "new_shared_otp_list"]
        fn new_ptr() -> *mut SharedOneTimePasswordList;
    }

    impl cxx_qt::Threading for OneTimePasswords {}
    impl cxx_qt::Initialize for OneTimePasswords {}

    #[qenum(SharedOneTimePasswordList)]
    enum SharedOneTimePasswordListRoles {
        Id,
        AccountName,
        AccountIssuer,
        SecretKey,
        CanEdit,
        IconSource,
        SharedByName,
    }

    #[qenum(MyOneTimePasswordList)]
    enum MyOneTimePasswordListRoles {
        Id,
        AccountName,
        AccountIssuer,
        SecretKey,
        CanEdit,
        IconSource,
        SharedWithEmails,
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        type SharedOneTimePasswordList = super::SharedOneTimePasswordListStruct;

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
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        type MyOneTimePasswordList = super::MyOneTimePasswordListStruct;

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
    }

    #[auto_cxx_name]
    #[auto_rust_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(
            *mut MyOneTimePasswordList,
            my_one_time_passwords,
            cxx_name = "myOneTimePasswords"
        )]
        #[qproperty(
            *mut SharedOneTimePasswordList,
            shared_one_time_passwords,
            cxx_name = "sharedOneTimePasswords"
        )]
        #[qproperty(bool, loading)]
        #[qproperty(QStringList, shared_by_names, cxx_name = "sharedByNames")]
        type OneTimePasswords = super::OneTimePasswordsStruct;

        #[qinvokable]
        fn loadOneTimePasswords(self: Pin<&mut Self>);

        #[qinvokable]
        fn shareOtp(self: Pin<&mut Self>, otp_id: i64, shared_with_emails: QStringList);

        #[qinvokable]
        fn editOtp(self: Pin<&mut Self>, otp_id: i64, account_name: QString);

        #[qinvokable]
        fn deleteOtp(self: Pin<&mut Self>, otp_id: i64);
    }
}

#[derive(Default)]
pub struct SharedOneTimePasswordListStruct {
    pub otps: Vec<api::otp::SharedOneTimePassword>,
}

#[derive(Default)]
pub struct MyOneTimePasswordListStruct {
    pub otps: Vec<api::otp::MyOneTimePassword>,
}

pub struct OneTimePasswordsStruct {
    my_one_time_passwords: *mut ffi::MyOneTimePasswordList,
    shared_one_time_passwords: *mut ffi::SharedOneTimePasswordList,
    loading: bool,
    shared_by_names: QStringList,
}

impl Default for OneTimePasswordsStruct {
    fn default() -> Self {
        Self {
            my_one_time_passwords: std::ptr::null_mut(),
            shared_one_time_passwords: std::ptr::null_mut(),
            loading: false,
            shared_by_names: QStringList::default(),
        }
    }
}

unsafe impl Send for OneTimePasswordsStruct {}

impl cxx_qt::Initialize for ffi::OneTimePasswords {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut()
            .set_my_one_time_passwords(ffi::new_my_otp_list());
        self.as_mut()
            .set_shared_one_time_passwords(ffi::new_shared_otp_list());
    }
}

impl ffi::SharedOneTimePasswordList {
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.otps.len() as i32
    }

    fn role_names(&self) -> ffi::QHash_i32_QByteArray {
        let mut hash = ffi::QHash_i32_QByteArray::default();
        hash.insert(ffi::SharedOneTimePasswordListRoles::Id.repr, "otpId".into());
        hash.insert(
            ffi::SharedOneTimePasswordListRoles::AccountName.repr,
            "accountName".into(),
        );
        hash.insert(
            ffi::SharedOneTimePasswordListRoles::AccountIssuer.repr,
            "accountIssuer".into(),
        );
        hash.insert(
            ffi::SharedOneTimePasswordListRoles::SecretKey.repr,
            "secretKey".into(),
        );
        hash.insert(
            ffi::SharedOneTimePasswordListRoles::CanEdit.repr,
            "canEdit".into(),
        );
        hash.insert(
            ffi::SharedOneTimePasswordListRoles::IconSource.repr,
            "iconSource".into(),
        );
        hash.insert(
            ffi::SharedOneTimePasswordListRoles::SharedByName.repr,
            "sharedByName".into(),
        );
        hash
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = ffi::SharedOneTimePasswordListRoles { repr: role };

        if let Some(api::otp::SharedOneTimePassword {
            id,
            account_name,
            account_issuer,
            secret_key,
            can_edit,
            brand_icon,
            simple_icon,
            brand_icon_similarity,
            simple_icon_similarity,
            shared_by,
        }) = self.otps.get(index.row() as usize)
        {
            match role {
                ffi::SharedOneTimePasswordListRoles::Id => {
                    return id.into();
                }
                ffi::SharedOneTimePasswordListRoles::AccountName => {
                    return (&QString::from(account_name)).into();
                }
                ffi::SharedOneTimePasswordListRoles::AccountIssuer => {
                    return (&QString::from(account_issuer)).into();
                }
                ffi::SharedOneTimePasswordListRoles::SecretKey => {
                    return (&QString::from(secret_key)).into();
                }
                ffi::SharedOneTimePasswordListRoles::CanEdit => {
                    return can_edit.into();
                }
                ffi::SharedOneTimePasswordListRoles::IconSource => {
                    let ref icon = get_icon_source(
                        *brand_icon_similarity,
                        *simple_icon_similarity,
                        brand_icon.to_string(),
                        simple_icon.to_string(),
                    );
                    return icon.into();
                }
                ffi::SharedOneTimePasswordListRoles::SharedByName => {
                    return (&QString::from(shared_by.name.clone())).into();
                }
                _ => {}
            }
        }
        QVariant::default()
    }
}

impl ffi::MyOneTimePasswordList {
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.otps.len() as i32
    }

    fn role_names(&self) -> ffi::QHash_i32_QByteArray {
        let mut hash = ffi::QHash_i32_QByteArray::default();
        hash.insert(ffi::MyOneTimePasswordListRoles::Id.repr, "otpId".into());
        hash.insert(
            ffi::MyOneTimePasswordListRoles::AccountName.repr,
            "accountName".into(),
        );
        hash.insert(
            ffi::MyOneTimePasswordListRoles::AccountIssuer.repr,
            "accountIssuer".into(),
        );
        hash.insert(
            ffi::MyOneTimePasswordListRoles::SecretKey.repr,
            "secretKey".into(),
        );
        hash.insert(
            ffi::MyOneTimePasswordListRoles::CanEdit.repr,
            "canEdit".into(),
        );
        hash.insert(
            ffi::MyOneTimePasswordListRoles::IconSource.repr,
            "iconSource".into(),
        );
        hash.insert(
            ffi::MyOneTimePasswordListRoles::SharedWithEmails.repr,
            "sharedWithEmails".into(),
        );
        hash
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = ffi::MyOneTimePasswordListRoles { repr: role };

        if let Some(api::otp::MyOneTimePassword {
            id,
            account_name,
            account_issuer,
            secret_key,
            can_edit,
            brand_icon,
            simple_icon,
            brand_icon_similarity,
            simple_icon_similarity,
            shared_with,
        }) = self.otps.get(index.row() as usize)
        {
            match role {
                ffi::MyOneTimePasswordListRoles::Id => return id.into(),
                ffi::MyOneTimePasswordListRoles::AccountName => {
                    return (&QString::from(account_name)).into();
                }
                ffi::MyOneTimePasswordListRoles::AccountIssuer => {
                    return (&QString::from(account_issuer)).into();
                }
                ffi::MyOneTimePasswordListRoles::SecretKey => {
                    return (&QString::from(secret_key)).into();
                }
                ffi::MyOneTimePasswordListRoles::CanEdit => return can_edit.into(),
                ffi::MyOneTimePasswordListRoles::IconSource => {
                    let ref icon = get_icon_source(
                        *brand_icon_similarity,
                        *simple_icon_similarity,
                        brand_icon.to_string(),
                        simple_icon.to_string(),
                    );
                    return icon.into();
                }
                ffi::MyOneTimePasswordListRoles::SharedWithEmails => {
                    let mut list = QStringList::default();
                    for shared_with in shared_with {
                        list.append(shared_with.email.clone().into());
                    }

                    return (&list).into();
                }
                _ => {}
            }
        }
        QVariant::default()
    }
}

macro_rules! with_model {
    ($ptr:expr, |$pin:ident| $body:expr) => {
        if let Some(inner) = unsafe { $ptr.as_mut() } {
            let mut $pin = unsafe { Pin::new_unchecked(inner) };
            $body
        }
    };
}

impl ffi::OneTimePasswords {
    fn load_one_time_passwords(mut self: Pin<&mut Self>) {
        self.as_mut().set_loading(true);

        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            if let Ok(otps) = api::otp::get_one_time_passwords().await {
                let shared_names = otps
                    .shared_one_time_passwords
                    .iter()
                    .map(|otp| otp.shared_by.name.clone())
                    .collect::<BTreeSet<_>>();
                let shared_otps = otps.shared_one_time_passwords;
                let my_otps = otps.my_one_time_passwords;
                qt_thread
                    .queue(move |mut otps| {
                        let mut names = QStringList::default();
                        for name in shared_names.iter() {
                            names.append(name.into());
                        }
                        otps.as_mut().set_shared_by_names(names);

                        with_model!(*otps.as_mut().my_one_time_passwords(), |model| {
                            model.as_mut().begin_reset_model();
                            model.as_mut().rust_mut().otps = my_otps;
                            model.as_mut().end_reset_model();
                        });
                        // if let Some(inner) = unsafe { (*otps.as_mut().shared_one_time_passwords()).as_mut() } {
                        //     let mut model = unsafe { Pin::new_unchecked(inner) };
                        //     model.as_mut().begin_reset_model();
                        //     model.as_mut().rust_mut().otps = shared_otps;
                        //     model.as_mut().end_reset_model();
                        // }
                        with_model!(*otps.as_mut().shared_one_time_passwords(), |model| {
                            model.as_mut().begin_reset_model();
                            model.as_mut().rust_mut().otps = shared_otps;
                            model.as_mut().end_reset_model();
                        });

                        otps.as_mut().set_loading(false);
                    })
                    .unwrap();
            }
        });
    }

    fn share_otp(self: Pin<&mut Self>, otp_id: i64, shared_with_emails: QStringList) {
        let emails = shared_with_emails
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            if let Ok(owners) = get_owners().await {
                let ids = owners
                    .iter()
                    .filter(|o| emails.contains(&o.email))
                    .map(|owner| owner.id)
                    .collect::<Vec<i64>>();
                if api::otp::share_one_time_password(otp_id, ids).await.is_ok() {
                    if let Ok(otps) = api::otp::get_one_time_passwords().await {
                        let my_otps = otps.my_one_time_passwords;
                        qt_thread
                            .queue(move |mut otps| {
                                with_model!(*otps.as_mut().my_one_time_passwords(), |model| {
                                    model.as_mut().begin_reset_model();
                                    model.as_mut().rust_mut().otps = my_otps;
                                    model.as_mut().end_reset_model();
                                });
                            })
                            .unwrap();
                    }
                }
            }
        });
    }

    fn edit_otp(self: Pin<&mut Self>, otp_id: i64, account_name: QString) {
        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            if api::otp::update_one_time_password(otp_id, account_name.into())
                .await
                .is_ok()
            {
                if let Ok(otps) = api::otp::get_one_time_passwords().await {
                    let my_otps = otps.my_one_time_passwords;
                    qt_thread
                        .queue(move |mut otps| {
                            with_model!(*otps.as_mut().my_one_time_passwords(), |model| {
                                model.as_mut().begin_reset_model();
                                model.as_mut().rust_mut().otps = my_otps;
                                model.as_mut().end_reset_model();
                            });
                        })
                        .unwrap();
                }
            }
        });
    }

    fn delete_otp(self: Pin<&mut Self>, otp_id: i64) {
        let qt_thread = self.qt_thread();
        tokio::spawn(async move {
            if api::otp::delete_one_time_password(otp_id).await.is_ok() {
                if let Ok(otps) = api::otp::get_one_time_passwords().await {
                    let my_otps = otps.my_one_time_passwords;
                    qt_thread
                        .queue(move |mut otps| {
                            with_model!(*otps.as_mut().my_one_time_passwords(), |model| {
                                model.as_mut().begin_reset_model();
                                model.as_mut().rust_mut().otps = my_otps;
                                model.as_mut().end_reset_model();
                            });
                        })
                        .unwrap();
                }
            }
        });
    }
}
