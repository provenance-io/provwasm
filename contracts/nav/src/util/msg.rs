use cosmwasm_std::Addr;

use crate::core::msg::{CategorizedSecurity, Security};

impl From<String> for Security {
    fn from(security: String) -> Self {
        let parts: Vec<&str> = security.split('.').collect();
        let category = parts[0].into();
        let mut name: Option<String> = None;
        if parts.len() > 1 {
            name = Some(parts[1].into());
        }
        Security { category, name }
    }
}

impl From<&Security> for String {
    fn from(security: &Security) -> Self {
        if security.name.is_none() {
            return security.category.to_string();
        }
        format!("{}.{}", security.category, security.name.as_ref().unwrap())
    }
}

impl From<(String, String)> for Security {
    fn from(security: (String, String)) -> Self {
        let category = security.0;
        let mut name: Option<String> = None;
        if !security.1.is_empty() {
            name = Some(security.1);
        }
        Security { category, name }
    }
}

impl Security {
    pub fn new(category: &str) -> Self {
        Self {
            category: category.to_string(),
            name: None,
        }
    }

    pub fn new_with_name(category: &str, name: &str) -> Self {
        Self {
            category: category.to_string(),
            name: Some(name.to_string()),
        }
    }
}

impl ToString for Security {
    fn to_string(&self) -> String {
        if self.name.is_some() {
            return format!("{}.{}", self.category, self.name.as_ref().unwrap());
        }
        self.category.to_string()
    }
}

impl From<(String, Addr)> for CategorizedSecurity {
    fn from(pair: (String, Addr)) -> Self {
        CategorizedSecurity {
            name: pair.0,
            asset: pair.1,
        }
    }
}
