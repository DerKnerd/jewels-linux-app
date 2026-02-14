use crate::api::owner;
use qmetaobject::{
    QObject, QPointer, SimpleListItem, SimpleListModel, qt_base_class, qt_method, qt_property,
    qt_signal,
};
use qttypes::{QByteArray, QString, QVariant};
use std::cell::RefCell;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Owner {
    base: qt_base_class!(trait QObject),

    idChanged: qt_signal!(),
    nameChanged: qt_signal!(),
    emailChanged: qt_signal!(),
    isAdminChanged: qt_signal!(),
    profilePictureChanged: qt_signal!(),

    pub id: qt_property!(i64; NOTIFY idChanged),
    pub name: qt_property!(QString; NOTIFY nameChanged),
    pub email: qt_property!(QString; NOTIFY emailChanged),
    pub isAdmin: qt_property!(bool; NOTIFY isAdminChanged),
    pub profilePicture: qt_property!(QString; NOTIFY profilePictureChanged),
}

impl From<owner::Owner> for Owner {
    fn from(value: owner::Owner) -> Self {
        Self {
            base: Default::default(),

            idChanged: Default::default(),
            nameChanged: Default::default(),
            emailChanged: Default::default(),
            isAdminChanged: Default::default(),
            profilePictureChanged: Default::default(),

            id: value.id,
            name: QString::from(value.name),
            email: QString::from(value.email),
            isAdmin: value.is_admin,
            profilePicture: QString::from(value.profile_picture),
        }
    }
}

impl SimpleListItem for Owner {
    fn get(&self, role: i32) -> QVariant {
        match role {
            0 => self.id.clone().into(),
            1 => self.name.clone().into(),
            2 => self.email.clone().into(),
            3 => self.isAdmin.into(),
            4 => self.profilePicture.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn names() -> Vec<QByteArray> {
        vec![
            "id".into(),
            "name".into(),
            "email".into(),
            "isAdmin".into(),
            "profilePicture".into(),
        ]
    }
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Owners {
    base: qt_base_class!(trait QObject),

    pub owners: qt_property!(RefCell<SimpleListModel<Owner>>; CONST),
    pub load: qt_method!(
        fn load(&mut self) {
            self.load_owner();
        }
    ),
}

impl Owners {
    fn load_owner(&mut self) {
        let qptr = QPointer::from(&*self);
        let set_owner = qmetaobject::queued_callback(move |owners: Vec<owner::Owner>| {
            if let Some(this) = qptr.as_pinned() {
                let owners_ref = this.borrow_mut();
                owners_ref
                    .owners
                    .borrow_mut()
                    .reset_data(owners.into_iter().map(Into::into).collect());
            }
        });
        tokio::spawn(async move {
            let owners = owner::get_owners().await.unwrap_or_default();
            set_owner(owners);
        });
    }
}
