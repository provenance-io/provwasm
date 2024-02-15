use cosmwasm_std::{Addr, Storage, Uint64};
use cw_storage_plus::{Bound, Map};

use crate::core::{
    constants::{
        ASSET_SECURITY_KEY, DEFAULT_WITH_SECURITY_LIMIT, MAX_WITH_SECURITY_LIMIT,
        SECURITY_TO_ASSET_KEY,
    },
    error::ContractError,
    msg::{Paginate, Security},
};

pub const ASSET_TO_SECURITY: Map<&Addr, Security> = Map::new(ASSET_SECURITY_KEY);
pub const SECURITY_TO_ASSET: Map<((&str, &str), &Addr), ()> = Map::new(SECURITY_TO_ASSET_KEY);

/// Attempts to get the asset's security that is stored within the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to retrieve the tag of.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// get_security(deps.storage, &addr)?;
/// `
pub fn get_security(storage: &dyn Storage, asset_addr: &Addr) -> Result<Security, ContractError> {
    Ok(ASSET_TO_SECURITY.load(storage, asset_addr)?)
}

/// Attempts to get all assets that have the specified security.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `security` - The security to do a lookup by.
/// * `paginate` - Struct containing optional pagination args.
///
/// # Examples
/// ```
/// with_security(deps.storage, Security::new("security"), Paginate{limit: None, start_after: None})?;
/// `
pub fn with_security(
    storage: &dyn Storage,
    security: &Security,
    paginate: Paginate<Addr>,
) -> Result<Vec<Addr>, ContractError> {
    let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
    let start = paginate.start_after.as_ref().map(Bound::exclusive);
    let limit = paginate
        .limit
        .unwrap_or(Uint64::new(DEFAULT_WITH_SECURITY_LIMIT))
        .min(Uint64::new(MAX_WITH_SECURITY_LIMIT))
        .u64() as usize;
    let assets: Result<Vec<Addr>, ContractError> = SECURITY_TO_ASSET
        .prefix(key)
        .keys(storage, start, None, cosmwasm_std::Order::Ascending)
        .map(|result| result.map_err(ContractError::Std))
        .take(limit)
        .collect();
    assets
}

/// Attempts to check if any assets has the supplied security.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `security` - The security to look for.
///
/// # Examples
/// ```
/// has_security(deps.storage, Security::new("tag"))?;
/// `
pub fn has_security(storage: &dyn Storage, security: &Security) -> bool {
    let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
    !SECURITY_TO_ASSET.prefix(key).is_empty(storage)
}

/// Attempts to set the asset's security in the contract's storage.
/// An entry will be placed into a lookup table and reverse lookup table.
/// The previous entry in the reverse lookup table will also be removed.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to attach a security to.
/// * `security` - The security to attach to the to the asset.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// set_security(deps.as_mut().storage, &addr, Security::new("tag"))?;
/// `
pub fn set_security(
    storage: &mut dyn Storage,
    asset_addr: &Addr,
    security: &Security,
) -> Result<(), ContractError> {
    let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
    remove_security(storage, asset_addr);
    ASSET_TO_SECURITY.save(storage, asset_addr, security)?;
    Ok(SECURITY_TO_ASSET.save(storage, (key, asset_addr), &())?)
}

/// Removes the asset's security from the contract's storage.
/// An entry will be removed from the lookup table and reverse lookup table.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to remove the security from.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// remove_security(deps.as_mut().storage, &addr);
/// `
pub fn remove_security(storage: &mut dyn Storage, asset_addr: &Addr) {
    let security = get_security(storage, asset_addr);
    ASSET_TO_SECURITY.remove(storage, asset_addr);
    if let Ok(security_to_remove) = security {
        let key: (&str, &str) = (
            &security_to_remove.category,
            &security_to_remove.name.unwrap_or_default(),
        );
        SECURITY_TO_ASSET.remove(storage, (key, asset_addr));
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint64};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::{Paginate, Security},
        storage::asset::{
            has_security, remove_security, set_security, with_security, ASSET_TO_SECURITY,
            SECURITY_TO_ASSET,
        },
    };

    use super::get_security;

    #[test]
    fn test_get_security_empty() {
        let deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = get_security(&deps.storage, &asset_addr);
        security.expect_err("should throw an error when asset is missing");
    }

    #[test]
    fn test_has_security_missing() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        set_security(deps.as_mut().storage, &asset_addr, &Security::new("tag1"))
            .expect("should be successful");
        let value = has_security(&deps.storage, &Security::new("tag2"));
        let expected = false;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_security_success() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        let value = has_security(&deps.storage, &security);
        let expected = true;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_security_with_name_success() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        let value = has_security(&deps.storage, &security);
        let expected = true;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_set_security_single() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security);

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect("should have entry in TAG_TO_ASSET")
    }

    #[test]
    fn test_set_security_single_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security);

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect("should have entry in TAG_TO_ASSET")
    }

    #[test]
    fn test_set_security_duplicate() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");
        let security2 = Security::new("tag2");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        let key2: (&str, &str) = (&security2.category, &security2.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr, &security2).expect("should be successful");

        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security2);

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect_err("should remove original entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key2, &asset_addr))
            .expect("should have latest entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_set_security_multiple() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");
        let security2 = Security::new("tag2");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        let key2: (&str, &str) = (&security2.category, &security2.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");

        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security);

        let loaded_security2 = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr2)
            .expect("should have both entries in ASSET_TO_SECURITY");
        assert_eq!(loaded_security2, security2);

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect("should have entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr2))
            .expect("should have both entries in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_set_security_multiple_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new_with_name("tag1", "name");
        let security2 = Security::new_with_name("tag1", "name2");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        let key2: (&str, &str) = (&security2.category, &security2.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");

        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security);

        let loaded_security2 = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr2)
            .expect("should have both entries in ASSET_TO_SECURITY");
        assert_eq!(loaded_security2, security2);

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect("should have entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr2))
            .expect("should have both entries in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_invalid() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr2);

        let loaded_tag = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_tag, security);
        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect("should have entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_single() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr);

        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect_err("should have no entry in ASSET_TO_SECURITY");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect_err("should have no entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_single_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr);

        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect_err("should have no entry in ASSET_TO_SECURITY");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect_err("should have no entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_multiple() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security: Security = Security::new("tag1");
        let security2 = Security::new("tag2");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        let key2: (&str, &str) = (&security2.category, &security2.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr);
        remove_security(deps.as_mut().storage, &asset_addr2);

        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect_err("should not have entry in ASSET_TO_SECURITY");
        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr2)
            .expect_err("should remove both entries from ASSET_TO_SECURITY");

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect_err("should not have entry in TAG_TO_ASSET");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key2, &asset_addr2))
            .expect_err("should remove both entries frrom TAG_TO_ASSET");
    }

    #[test]
    fn test_remove_multiple_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security: Security = Security::new_with_name("tag1", "name");
        let security2 = Security::new_with_name("tag1", "name2");
        let key: (&str, &str) = (&security.category, &security.name.unwrap_or_default());
        let key2: (&str, &str) = (&security2.category, &security2.name.unwrap_or_default());

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr);
        remove_security(deps.as_mut().storage, &asset_addr2);

        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect_err("should not have entry in ASSET_TO_SECURITY");
        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr2)
            .expect_err("should remove both entries from ASSET_TO_SECURITY");

        SECURITY_TO_ASSET
            .load(&deps.storage, (key, &asset_addr))
            .expect_err("should not have entry in TAG_TO_ASSET");
        SECURITY_TO_ASSET
            .load(&deps.storage, (key2, &asset_addr2))
            .expect_err("should remove both entries frrom TAG_TO_ASSET");
    }

    #[test]
    fn test_with_security_empty() {
        let deps = mock_provenance_dependencies();
        let expected: Vec<Addr> = vec![];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let tags = with_security(&deps.storage, &Security::new("tag1"), paginate)
            .expect("should successfully obtain tags");
        assert_eq!(expected, tags);
    }

    #[test]
    fn test_with_security_one_security() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain tags");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_security_one_security_with_name() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain tags");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_security_multi_asset_same_security() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test"), Addr::unchecked("test2")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security).expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_security_multi_asset_same_security_with_different_name() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new_with_name("tag1", "name");
        let security2 = Security::new_with_name("tag1", "name2");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_security_multi_asset_different_security() {
        let mut deps = mock_provenance_dependencies();
        let expected1 = vec![Addr::unchecked("test")];
        let expected2 = vec![Addr::unchecked("test2")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");
        let security2 = Security::new("tag2");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate.clone())
            .expect("should successfully obtain securities");
        assert_eq!(expected1, securities);

        let securities = with_security(&deps.storage, &security2, paginate.clone())
            .expect("should successfully obtain tags");
        assert_eq!(expected2, securities);
    }

    #[test]
    fn test_paginate_limit() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test")];
        let paginate = Paginate {
            limit: Some(Uint64::new(1)),
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security).expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_paginate_start_after() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test2")];
        let paginate = Paginate {
            limit: None,
            start_after: Some(Addr::unchecked("test")),
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security).expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_paginate_default_limit() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![
            Addr::unchecked("test01"),
            Addr::unchecked("test02"),
            Addr::unchecked("test03"),
            Addr::unchecked("test04"),
            Addr::unchecked("test05"),
            Addr::unchecked("test06"),
            Addr::unchecked("test07"),
            Addr::unchecked("test08"),
            Addr::unchecked("test09"),
            Addr::unchecked("test10"),
        ];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let security = Security::new("tag1");
        let asset_addr11 = Addr::unchecked("test11");
        for asset_addr in &expected {
            set_security(deps.as_mut().storage, &asset_addr, &security)
                .expect("should be successful");
        }
        set_security(deps.as_mut().storage, &asset_addr11, &security)
            .expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain tags");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_paginate_max_limit() {
        let mut deps = mock_provenance_dependencies();
        let mut expected: Vec<Addr> = vec![];
        for i in 1..101 {
            let padded = format!("test{:03}", i);
            expected.push(Addr::unchecked(padded));
        }
        let paginate = Paginate {
            limit: Some(Uint64::new(101)),
            start_after: None,
        };
        let security = Security::new("tag1");
        let remainder = Addr::unchecked("test101");
        for asset_addr in &expected {
            set_security(deps.as_mut().storage, &asset_addr, &security)
                .expect("should be successful");
        }
        set_security(deps.as_mut().storage, &remainder, &security).expect("should be successful");

        let securities = with_security(&deps.storage, &security, paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }
}
