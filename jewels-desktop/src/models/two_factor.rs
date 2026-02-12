use crate::api::otp;
use crate::api::owner::get_owners;
use crate::models::Owner;
use qmetaobject::{
    QObject, QPointer, SimpleListItem, SimpleListModel, qt_base_class, qt_method, qt_property,
    qt_signal,
};
use qttypes::{QByteArray, QString, QStringList, QVariant, QVariantList, QVariantMap};
use std::cell::RefCell;
use std::collections::BTreeMap;

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

#[derive(QObject, Default)]
#[allow(non_snake_case)]
pub struct OneTimePassword {
    base: qt_base_class!(trait QObject),

    idChanged: qt_signal!(),
    accountNameChanged: qt_signal!(),
    accountIssuerChanged: qt_signal!(),
    secretKeyChanged: qt_signal!(),
    canEditChanged: qt_signal!(),
    brandIconChanged: qt_signal!(),
    simpleIconChanged: qt_signal!(),
    brandIconSimilarityChanged: qt_signal!(),
    simpleIconSimilarityChanged: qt_signal!(),

    pub id: qt_property!(i64; NOTIFY idChanged),
    pub accountName: qt_property!(QString; NOTIFY accountNameChanged),
    pub accountIssuer: qt_property!(QString; NOTIFY accountIssuerChanged),
    pub secretKey: qt_property!(QString; NOTIFY secretKeyChanged),
    pub canEdit: qt_property!(bool; NOTIFY canEditChanged),
    pub brandIcon: qt_property!(QString; NOTIFY brandIconChanged),
    pub simpleIcon: qt_property!(QString; NOTIFY simpleIconChanged),
    pub brandIconSimilarity: qt_property!(f64; NOTIFY brandIconSimilarityChanged),
    pub simpleIconSimilarity: qt_property!(f64; NOTIFY simpleIconSimilarityChanged),

    pub sharedWith: qt_property!(RefCell<SimpleListModel<Owner>>; CONST),
    pub sharedWithEmails: qt_property!(QStringList; CONST),

    pub iconSource: qt_property!(QString),
}

impl From<otp::OneTimePassword> for OneTimePassword {
    fn from(value: otp::OneTimePassword) -> Self {
        let shared_with_qt = value.shared_with.into_iter().map(Owner::from);

        // Collect emails for easy QML comparisons
        let mut emails = QStringList::default();
        for u in shared_with_qt.clone() {
            emails.push(u.email.clone());
        }

        Self {
            base: Default::default(),

            idChanged: Default::default(),
            accountNameChanged: Default::default(),
            accountIssuerChanged: Default::default(),
            secretKeyChanged: Default::default(),
            canEditChanged: Default::default(),
            brandIconChanged: Default::default(),
            simpleIconChanged: Default::default(),
            brandIconSimilarityChanged: Default::default(),
            simpleIconSimilarityChanged: Default::default(),

            id: value.id,
            accountName: QString::from(value.account_name),
            accountIssuer: QString::from(value.account_issuer),
            secretKey: QString::from(value.secret_key),
            canEdit: value.can_edit,
            brandIcon: QString::from(value.brand_icon.clone()),
            simpleIcon: QString::from(value.simple_icon.clone()),
            brandIconSimilarity: value.brand_icon_similarity,
            simpleIconSimilarity: value.simple_icon_similarity,

            sharedWith: RefCell::new(SimpleListModel::from_iter(shared_with_qt)),
            sharedWithEmails: emails,

            iconSource: get_icon_source(
                value.brand_icon_similarity,
                value.simple_icon_similarity,
                value.brand_icon,
                value.simple_icon,
            ),
        }
    }
}

impl SimpleListItem for OneTimePassword {
    fn get(&self, role: i32) -> QVariant {
        match role {
            0 => self.id.clone().into(),
            1 => self.accountName.clone().into(),
            2 => self.accountIssuer.clone().into(),
            3 => self.secretKey.clone().into(),
            4 => self.canEdit.into(),
            5 => self.iconSource.clone().into(),
            6 => self.sharedWithEmails.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn names() -> Vec<QByteArray> {
        vec![
            "otpId".into(),
            "accountName".into(),
            "accountIssuer".into(),
            "secretKey".into(),
            "canEdit".into(),
            "iconSource".into(),
            "sharedWithEmails".into(),
        ]
    }
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct SharedOneTimePassword {
    base: qt_base_class!(trait QObject),

    idChanged: qt_signal!(),
    accountNameChanged: qt_signal!(),
    accountIssuerChanged: qt_signal!(),
    secretKeyChanged: qt_signal!(),
    canEditChanged: qt_signal!(),
    brandIconChanged: qt_signal!(),
    simpleIconChanged: qt_signal!(),
    brandIconSimilarityChanged: qt_signal!(),
    simpleIconSimilarityChanged: qt_signal!(),

    pub id: qt_property!(i64; NOTIFY idChanged),
    pub accountName: qt_property!(QString; NOTIFY accountNameChanged),
    pub accountIssuer: qt_property!(QString; NOTIFY accountIssuerChanged),
    pub secretKey: qt_property!(QString; NOTIFY secretKeyChanged),
    pub canEdit: qt_property!(bool; NOTIFY canEditChanged),
    pub brandIcon: qt_property!(QString; NOTIFY brandIconChanged),
    pub simpleIcon: qt_property!(QString; NOTIFY simpleIconChanged),
    pub brandIconSimilarity: qt_property!(f64; NOTIFY brandIconSimilarityChanged),
    pub simpleIconSimilarity: qt_property!(f64; NOTIFY simpleIconSimilarityChanged),
    pub sharedBy: qt_property!(RefCell<Owner>; CONST),

    pub iconSource: qt_property!(QString),
}

impl From<otp::SharedOneTimePassword> for SharedOneTimePassword {
    fn from(value: otp::SharedOneTimePassword) -> Self {
        Self {
            base: Default::default(),

            idChanged: Default::default(),
            accountNameChanged: Default::default(),
            accountIssuerChanged: Default::default(),
            secretKeyChanged: Default::default(),
            canEditChanged: Default::default(),
            brandIconChanged: Default::default(),
            simpleIconChanged: Default::default(),
            brandIconSimilarityChanged: Default::default(),
            simpleIconSimilarityChanged: Default::default(),

            id: value.id,
            accountName: QString::from(value.account_name),
            accountIssuer: QString::from(value.account_issuer),
            secretKey: QString::from(value.secret_key),
            canEdit: value.can_edit,
            brandIcon: QString::from(value.brand_icon.clone()),
            simpleIcon: QString::from(value.simple_icon.clone()),
            brandIconSimilarity: value.brand_icon_similarity,
            simpleIconSimilarity: value.simple_icon_similarity,

            sharedBy: RefCell::new(value.shared_by.into()),

            iconSource: get_icon_source(
                value.brand_icon_similarity,
                value.simple_icon_similarity,
                value.brand_icon,
                value.simple_icon,
            ),
        }
    }
}

impl SimpleListItem for SharedOneTimePassword {
    fn get(&self, role: i32) -> QVariant {
        match role {
            0 => self.id.clone().into(),
            1 => self.accountName.clone().into(),
            2 => self.accountIssuer.clone().into(),
            3 => self.secretKey.clone().into(),
            4 => self.canEdit.into(),
            5 => self.iconSource.clone().into(),
            6 => self.sharedBy.borrow().name.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn names() -> Vec<QByteArray> {
        vec![
            "id".into(),
            "accountName".into(),
            "accountIssuer".into(),
            "secretKey".into(),
            "canEdit".into(),
            "iconSource".into(),
            "sharedByName".into(),
        ]
    }
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct OneTimePasswords {
    base: qt_base_class!(trait QObject),

    loadingChanged: qt_signal!(),
    sharedOneTimePasswordsChanged: qt_signal!(),

    pub myOneTimePasswords: qt_property!(RefCell<SimpleListModel<OneTimePassword>>; CONST),
    pub sharedOneTimePasswords: qt_property!(QVariantList; NOTIFY sharedOneTimePasswordsChanged),

    pub loading: qt_property!(bool; NOTIFY loadingChanged),

    pub loadOneTimePasswords: qt_method!(
        fn loadOneTimePasswords(&mut self) {
            self.load_one_time_password();
        }
    ),
    pub shareOtp: qt_method!(
        fn shareOtp(&mut self, otpId: i64, sharedWithEmails: QStringList) {
            self.share_otp(otpId, sharedWithEmails);
        }
    ),
    pub editOtp: qt_method!(
        fn editOtp(&mut self, otpId: i64, accountName: QString) {
            self.edit_otp(otpId, accountName);
        }
    ),
    pub deleteOtp: qt_method!(
        fn deleteOtp(&mut self, otpId: i64) {
            self.delete_otp(otpId);
        }
    ),
}

impl OneTimePasswords {
    fn get_shared_otps_as_map(shared_otp: Vec<otp::SharedOneTimePassword>) -> QVariantList {
        // Replace this with however you access the selected/shared OTP list in Rust:
        let otps = shared_otp.as_slice();

        // BTreeMap keeps groups sorted by name; use HashMap if you don't care about order.
        let mut groups = BTreeMap::<String, Vec<&otp::SharedOneTimePassword>>::new();

        for otp in otps {
            let key = otp.shared_by.name.clone(); // group key: sharedByName
            groups.entry(key).or_default().push(otp);
        }

        let mut out = QVariantList::default();

        for (name, entries) in groups {
            let mut otp_list = QVariantList::default();

            for otp in entries {
                let mut otp_map = QVariantMap::default();
                otp_map.insert("id".into(), QVariant::from(otp.id));
                otp_map.insert(
                    "accountName".into(),
                    QString::from(otp.account_name.clone()).into(),
                );
                otp_map.insert(
                    "accountIssuer".into(),
                    QString::from(otp.account_issuer.clone()).into(),
                );
                otp_map
                    .insert("secretKey".into(), QString::from(otp.secret_key.clone()).into());
                otp_map.insert("canEdit".into(), QVariant::from(otp.can_edit));
                otp_map.insert(
                    "iconSource".into(),
                    QVariant::from(get_icon_source(
                        otp.brand_icon_similarity,
                        otp.simple_icon_similarity,
                        otp.brand_icon.clone(),
                        otp.simple_icon.clone(),
                    ))
                        .clone(),
                );
                otp_map.insert(
                    "sharedByName".into(),
                    QString::from(otp.shared_by.name.clone()).into(),
                );

                otp_list.push(QVariant::from(otp_map));
            }

            let mut group_map = QVariantMap::default();
            group_map.insert("name".into(), QString::from(name).into());
            group_map.insert("otpCodes".into(), QVariant::from(otp_list));

            out.push(QVariant::from(group_map));
        }

        out
    }

    fn load_one_time_password(&mut self) {
        self.loading = true;
        self.loadingChanged();

        let qptr = QPointer::from(&*self);
        let set_otp = qmetaobject::queued_callback(
            move |(my_otp, shared_otp): (
                Vec<otp::OneTimePassword>,
                Vec<otp::SharedOneTimePassword>,
            )| {
                if let Some(this) = qptr.as_pinned() {
                    let mut otps_ref = this.borrow_mut();
                    otps_ref.loading = false;
                    otps_ref.loadingChanged();
                    let groups = Self::get_shared_otps_as_map(shared_otp);
                    otps_ref.sharedOneTimePasswords = groups;
                    otps_ref.sharedOneTimePasswordsChanged();
                    let mut my_otps = otps_ref.myOneTimePasswords.borrow_mut();
                    my_otps.reset_data(my_otp.into_iter().map(Into::into).collect());
                }
            },
        );
        tokio::spawn(async move {
            if let Ok(otps) = otp::get_one_time_passwords().await {
                set_otp((otps.my_one_time_passwords, otps.shared_one_time_passwords));
            }
        });
    }

    fn share_otp(&mut self, otp_id: i64, shared_with_emails: QStringList) {
        let qptr = QPointer::from(&*self);
        let set_otp = qmetaobject::queued_callback(
            move |(my_otp, shared_otp): (
                Vec<otp::OneTimePassword>,
                Vec<otp::SharedOneTimePassword>,
            )| {
                if let Some(this) = qptr.as_pinned() {
                    let mut otps_ref = this.borrow_mut();
                    otps_ref.loading = false;
                    otps_ref.loadingChanged();
                    let mut my_otps = otps_ref.myOneTimePasswords.borrow_mut();
                    my_otps.reset_data(my_otp.into_iter().map(Into::into).collect());
                }
            },
        );

        let emails = shared_with_emails
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        tokio::spawn(async move {
            if let Ok(owners) = get_owners().await {
                let ids = owners
                    .iter()
                    .filter(|o| emails.contains(&o.email))
                    .map(|owner| owner.id)
                    .collect::<Vec<i64>>();
                if otp::share_one_time_password(otp_id, ids).await.is_ok() {
                    if let Ok(otps) = otp::get_one_time_passwords().await {
                        set_otp((otps.my_one_time_passwords, otps.shared_one_time_passwords));
                    }
                }
            }
        });
    }

    fn edit_otp(&mut self, otp_id: i64, account_name: QString) {
        let qptr = QPointer::from(&*self);
        let set_otp = qmetaobject::queued_callback(
            move |my_otp: Vec<otp::OneTimePassword>| {
                if let Some(this) = qptr.as_pinned() {
                    let otps_ref = this.borrow_mut();
                    let mut my_otps = otps_ref.myOneTimePasswords.borrow_mut();
                    my_otps.reset_data(my_otp.into_iter().map(Into::into).collect());
                }
            },
        );

        tokio::spawn(async move {
            if otp::update_one_time_password(otp_id, account_name.into())
                .await
                .is_ok()
            {
                if let Ok(otps) = otp::get_one_time_passwords().await {
                    set_otp(otps.my_one_time_passwords);
                }
            }
        });
    }

    fn delete_otp(&mut self, otp_id: i64) {
        let qptr = QPointer::from(&*self);
        let set_otp = qmetaobject::queued_callback(
            move |my_otp: Vec<otp::OneTimePassword>| {
                if let Some(this) = qptr.as_pinned() {
                    let otps_ref = this.borrow_mut();
                    let mut my_otps = otps_ref.myOneTimePasswords.borrow_mut();
                    my_otps.reset_data(my_otp.into_iter().map(Into::into).collect());
                }
            },
        );

        tokio::spawn(async move {
            if otp::delete_one_time_password(otp_id).await.is_ok() {
                if let Ok(otps) = otp::get_one_time_passwords().await {
                    set_otp(otps.my_one_time_passwords);
                }
            }
        });
    }
}
