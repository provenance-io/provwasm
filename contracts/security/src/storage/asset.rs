use cosmwasm_std::{Addr, Empty, Storage, Uint64};
use cw_storage_plus::{Bound, Map};

use crate::core::{
    constants::{
        ASSET_SECURITY_KEY, DEFAULT_WITH_SECURITY_LIMIT, MAX_WITH_SECURITY_LIMIT,
        SECURITY_TO_ASSET_KEY,
    },
    error::ContractError,
    msg::{CategorizedSecurity, Paginate, Security},
};

pub const ASSET_TO_SECURITY: Map<&Addr, Security> = Map::new(ASSET_SECURITY_KEY);
pub const SECURITY_TO_ASSET: Map<(&str, &str, &Addr), Empty> = Map::new(SECURITY_TO_ASSET_KEY);

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
/// ```
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
/// ```
pub fn with_security(
    storage: &dyn Storage,
    security: &Security,
    paginate: Paginate<Addr>,
) -> Result<Vec<Addr>, ContractError> {
    let default_string = String::default();
    let prefix_key: (&str, &str) = (
        &security.category,
        &security.name.as_ref().unwrap_or(&default_string),
    );
    let start = paginate.start_after.as_ref().map(Bound::exclusive);
    let limit = paginate
        .limit
        .unwrap_or(Uint64::new(DEFAULT_WITH_SECURITY_LIMIT))
        .min(Uint64::new(MAX_WITH_SECURITY_LIMIT))
        .u64() as usize;
    let assets: Result<Vec<Addr>, ContractError> = SECURITY_TO_ASSET
        .prefix(prefix_key)
        .keys(storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .map(|result| result.map_err(ContractError::Std))
        .try_fold(Vec::new(), |mut vec, result| {
            vec.push(result?);
            Ok(vec)
        });
    assets
}

/// Attempts to get all assets that have the specified security category.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `category` - The security category to lookup addresses by.
/// * `paginate` - Struct containing optional pagination args.
///
/// # Examples
/// ```
/// with_security_category(deps.storage, "category", Paginate{limit: None, start_after: None})?;
/// ```
pub fn with_security_category(
    storage: &dyn Storage,
    category: &str,
    paginate: Paginate<CategorizedSecurity>,
) -> Result<Vec<CategorizedSecurity>, ContractError> {
    let start_after: Option<(&str, &Addr)> = paginate
        .start_after
        .as_ref()
        .map(|s| (s.name.as_str(), &s.asset));
    let start = start_after.map(Bound::exclusive);
    let limit = paginate
        .limit
        .unwrap_or(Uint64::new(DEFAULT_WITH_SECURITY_LIMIT))
        .min(Uint64::new(MAX_WITH_SECURITY_LIMIT))
        .u64() as usize;

    let assets: Result<Vec<CategorizedSecurity>, ContractError> = SECURITY_TO_ASSET
        .sub_prefix(category)
        .keys(storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .try_fold(Vec::new(), |mut vec, result| {
            vec.push(result?.into());
            Ok(vec)
        });
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
/// ```
pub fn has_security(storage: &dyn Storage, security: &Security) -> bool {
    let default_name = String::default();
    let key: (&str, &str) = (
        &security.category,
        &security.name.as_ref().unwrap_or(&default_name),
    );
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
/// ```
pub fn set_security(
    storage: &mut dyn Storage,
    asset_addr: &Addr,
    security: &Security,
) -> Result<(), ContractError> {
    remove_security(storage, asset_addr);
    ASSET_TO_SECURITY.save(storage, asset_addr, security)?;
    Ok(SECURITY_TO_ASSET.save(
        storage,
        (
            &security.category,
            &security.name.as_ref().unwrap_or(&"".to_string()),
            asset_addr,
        ),
        &Empty {},
    )?)
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
/// ```
pub fn remove_security(storage: &mut dyn Storage, asset_addr: &Addr) {
    let security = get_security(storage, asset_addr);
    ASSET_TO_SECURITY.remove(storage, asset_addr);
    if let Ok(security_to_remove) = security {
        SECURITY_TO_ASSET.remove(
            storage,
            (
                &security_to_remove.category,
                &security_to_remove.name.as_ref().unwrap_or(&"".to_string()),
                asset_addr,
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Empty, Uint64};
    use cw_storage_plus::Map;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{
            error::ContractError,
            msg::{CategorizedSecurity, Paginate, Security},
        },
        storage::asset::{
            has_security, remove_security, set_security, with_security, with_security_category,
            ASSET_TO_SECURITY, SECURITY_TO_ASSET,
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

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security);

        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect("should have entry in TAG_TO_ASSET");
    }

    #[test]
    fn test_set_security_single_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security);

        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect("should have entry in TAG_TO_ASSET");
    }

    #[test]
    fn test_set_security_duplicate() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");
        let security2 = Security::new("tag2");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr, &security2).expect("should be successful");

        let loaded_security = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_security, security2);

        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect_err("should remove original entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security2.category,
                    &security2.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect("should have latest entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_set_security_multiple() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");
        let security2 = Security::new("tag2");

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
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect("should have entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security2.category,
                    &security2.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr2,
                ),
            )
            .expect("should have both entries in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_set_security_multiple_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new_with_name("tag1", "name");
        let security2 = Security::new_with_name("tag1", "name2");

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
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect("should have entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security2.category,
                    &security2.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr2,
                ),
            )
            .expect("should have both entries in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_invalid() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new("tag1");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr2);

        let loaded_tag = ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_SECURITY");
        assert_eq!(loaded_tag, security);
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect("should have entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_single() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag1");
        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr);

        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect_err("should have no entry in ASSET_TO_SECURITY");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect_err("should have no entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_single_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");
        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        remove_security(deps.as_mut().storage, &asset_addr);

        ASSET_TO_SECURITY
            .load(&deps.storage, &asset_addr)
            .expect_err("should have no entry in ASSET_TO_SECURITY");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect_err("should have no entry in SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_multiple() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security: Security = Security::new("tag1");
        let security2 = Security::new("tag2");

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
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect_err("should not have entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security2.category,
                    &security2.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr2,
                ),
            )
            .expect_err("should remove both entries frrom SECURITY_TO_ASSET");
    }

    #[test]
    fn test_remove_multiple_with_name() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security: Security = Security::new_with_name("tag1", "name");
        let security2 = Security::new_with_name("tag1", "name2");

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
            .load(
                &deps.storage,
                (
                    &security.category,
                    &security.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr,
                ),
            )
            .expect_err("should not have entry in SECURITY_TO_ASSET");
        SECURITY_TO_ASSET
            .load(
                &deps.storage,
                (
                    &security2.category,
                    &security2.name.as_ref().unwrap_or(&"".to_string()),
                    &asset_addr2,
                ),
            )
            .expect_err("should remove both entries frrom SECURITY_TO_ASSET");
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

    pub const TEST_KEY: Map<(String, String, Addr), Empty> = Map::new("TEMP_MAP");
    #[test]
    fn test() {
        let mut deps = mock_provenance_dependencies();
        let key1 = (
            "category1".to_string(),
            "name1".to_string(),
            Addr::unchecked("addr1"),
        );
        let key2 = (
            "category1".to_string(),
            "name2".to_string(),
            Addr::unchecked("addr2"),
        );
        let key3 = (
            "category1".to_string(),
            "".to_string(),
            Addr::unchecked("addr3"),
        );
        let key4 = (
            "category1".to_string(),
            "name1".to_string(),
            Addr::unchecked("addr4"),
        );
        let key5 = (
            "category2".to_string(),
            "name1".to_string(),
            Addr::unchecked("addr5"),
        );
        let key6 = (
            "category2".to_string(),
            "name2".to_string(),
            Addr::unchecked("addr6"),
        );
        let key7 = (
            "category2".to_string(),
            "".to_string(),
            Addr::unchecked("addr7"),
        );
        let key8 = (
            "category2".to_string(),
            "name1".to_string(),
            Addr::unchecked("addr8"),
        );
        TEST_KEY
            .save(deps.as_mut().storage, key1, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key2, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key3, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key4, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key5, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key6, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key7, &Empty {})
            .unwrap();
        TEST_KEY
            .save(deps.as_mut().storage, key8, &Empty {})
            .unwrap();

        let res: Result<Vec<(String, Addr)>, ContractError> = TEST_KEY
            .sub_prefix("category1".to_string())
            .keys(&deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|result| result.map_err(ContractError::Std))
            .collect();

        println!("Found addresses:");
        for pair in &res.unwrap() {
            println!("{}", pair.1.to_string());
        }
    }

    #[test]
    fn test_with_security_category_empty() {
        let deps = mock_provenance_dependencies();
        let expected: Vec<CategorizedSecurity> = vec![];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let tags = with_security_category(&deps.storage, "category", paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, tags);
    }

    #[test]
    fn test_with_security_category_one_security() {
        let mut deps = mock_provenance_dependencies();
        let expected: Vec<CategorizedSecurity> =
            vec![("name".to_string(), Addr::unchecked("test")).into()];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let security = Security::new_with_name("tag1", "name");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");

        let securities = with_security_category(&deps.storage, "tag1", paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_security_category_multi_asset_same_category_with_different_name() {
        let mut deps = mock_provenance_dependencies();
        let expected: Vec<CategorizedSecurity> = vec![
            ("name".to_string(), Addr::unchecked("test")).into(),
            ("name2".to_string(), Addr::unchecked("test2")).into(),
        ];
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

        let securities = with_security_category(&deps.storage, "tag1", paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_security_category_multi_asset_different_category() {
        let mut deps = mock_provenance_dependencies();
        let expected1: Vec<CategorizedSecurity> =
            vec![("name".to_string(), Addr::unchecked("test")).into()];
        let expected2: Vec<CategorizedSecurity> =
            vec![("name2".to_string(), Addr::unchecked("test2")).into()];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new_with_name("tag1", "name");
        let security2 = Security::new_with_name("tag2", "name2");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security2)
            .expect("should be successful");

        let securities = with_security_category(&deps.storage, "tag1", paginate.clone())
            .expect("should successfully obtain securities");
        assert_eq!(expected1, securities);
        let securities = with_security_category(&deps.storage, "tag2", paginate)
            .expect("should successfully obtain other securities");
        assert_eq!(expected2, securities);
    }

    #[test]
    fn test_with_category_paginate_limit() {
        let mut deps = mock_provenance_dependencies();
        let expected: Vec<CategorizedSecurity> =
            vec![("name".to_string(), Addr::unchecked("test")).into()];
        let paginate = Paginate {
            limit: Some(Uint64::new(1)),
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new_with_name("tag1", "name");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security).expect("should be successful");

        let securities = with_security_category(&deps.storage, "tag1", paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_category_paginate_start_after() {
        let mut deps = mock_provenance_dependencies();
        let expected: Vec<CategorizedSecurity> =
            vec![("name".to_string(), Addr::unchecked("test2")).into()];
        let paginate = Paginate {
            limit: None,
            start_after: Some(("name".to_string(), Addr::unchecked("test")).into()),
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let security = Security::new_with_name("tag1", "name");

        set_security(deps.as_mut().storage, &asset_addr, &security).expect("should be successful");
        set_security(deps.as_mut().storage, &asset_addr2, &security).expect("should be successful");

        let securities = with_security_category(&deps.storage, "tag1", paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }

    #[test]
    fn test_with_category_paginate_default_limit() {
        let mut deps = mock_provenance_dependencies();
        let expected: Vec<CategorizedSecurity> = vec![
            ("".to_string(), Addr::unchecked("test01")).into(),
            ("".to_string(), Addr::unchecked("test02")).into(),
            ("".to_string(), Addr::unchecked("test03")).into(),
            ("".to_string(), Addr::unchecked("test04")).into(),
            ("".to_string(), Addr::unchecked("test05")).into(),
            ("".to_string(), Addr::unchecked("test06")).into(),
            ("".to_string(), Addr::unchecked("test07")).into(),
            ("".to_string(), Addr::unchecked("test08")).into(),
            ("".to_string(), Addr::unchecked("test09")).into(),
            ("".to_string(), Addr::unchecked("test10")).into(),
        ];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let security = Security::new("tag1");
        let asset_addr11 = Addr::unchecked("test11");
        for asset_addr in &expected {
            set_security(deps.as_mut().storage, &asset_addr.asset, &security)
                .expect("should be successful");
        }
        set_security(deps.as_mut().storage, &asset_addr11, &security)
            .expect("should be successful");

        let securities = with_security_category(&deps.storage, "tag1", paginate)
            .expect("should successfully obtain securities");
        assert_eq!(expected, securities);
    }
}
