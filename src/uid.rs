use snafu::Snafu;
use std::collections::HashMap;
use uuid::Uuid;

pub struct UidMap {
    pub root: String,
    pub map: HashMap<String, String>,
}

impl UidMap {
    pub fn new(uid_root: String) -> UidMap {
        UidMap {
            root: uid_root,
            map: HashMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, uid_key: &str) -> String {
        if let Some(value) = self.map.get(uid_key) {
            return value.clone();
        }
        let uid = self.generate_uid();
        self.map.insert(uid_key.to_owned(), uid.clone());
        uid
    }

    pub fn generate_uid(&self) -> String {
        generate_uid(&self.root)
    }
}

pub fn generate_uid(root: &str) -> String {
    let uid = Uuid::now_v7().to_u128_le();
    if root.ends_with(".") {
        format!("{0}{1}", root, uid)
    } else {
        format!("{0}.{1}", root, uid)
    }
}

pub fn validate_uid(uid: &str) -> Result<String, UIDRootError> {
    let uid = uid.to_string();

    if uid.chars().any(|c| !c.is_ascii_digit() && c != '.') {
        return InvalidCharacterSnafu { uid }.fail();
    }

    if uid.contains("..") {
        return ExtraPeriodSnafu { uid }.fail();
    }

    if uid.contains(".0") {
        return LeadingZeroSnafu { uid }.fail();
    }

    Ok(uid)
}

#[derive(Debug, Snafu)]
// #[snafu(visibility(pub(crate)))]
pub enum UIDRootError {
    #[snafu(display(
        "Invalid character in UID root `{}`, only period separated digits allowed",
        uid
    ))]
    InvalidCharacter { uid: String },
    #[snafu(display("Extra period in UID root `{}`", uid))]
    ExtraPeriod { uid: String },
    #[snafu(display("Leading zero in UID root `{}`", uid))]
    LeadingZero { uid: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uid() {
        let root = "1.2.3";
        let uid = generate_uid(root);
        assert!(uid.starts_with(root));
    }
}
