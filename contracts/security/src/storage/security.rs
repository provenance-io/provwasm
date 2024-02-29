use cosmwasm_std::{Empty, Storage, Uint64};
use cw_storage_plus::{Bound, Map};

use crate::core::{
    constants::{DEFAULT_SECURITY_TYPES_LIMIT, MAX_SECURITY_TYPES_LIMIT, SECURITY_TYPE_KEY},
    error::ContractError,
    msg::{Paginate, Security},
};

pub const SECURITY_TYPES: Map<(&str, &str), Empty> = Map::new(SECURITY_TYPE_KEY);

/// Attempts to add a security to the list of acceptable types in the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `security` - The security to add to the list of acceptable types.
///
/// # Examples
/// ```
/// add_type(deps.as_mut().storage, Security{category: "category".to_string(), name: None})?;
/// ```
pub fn add_type(storage: &mut dyn Storage, security: &Security) -> Result<(), ContractError> {
    let default_name = String::default();
    let key: (&str, &str) = (
        &security.category,
        &security.name.as_ref().unwrap_or(&default_name),
    );
    Ok(SECURITY_TYPES.save(storage, key, &Empty {})?)
}

/// Removes the security type from the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `security` - The security to remove from the list of acceptable types.
///
/// # Examples
/// ```
/// remove_type(deps.as_mut().storage, Security{category: "category".to_string(), name: None});
/// ```
pub fn remove_type(storage: &mut dyn Storage, security: &Security) {
    let default_name = String::default();
    let key: (&str, &str) = (
        &security.category,
        &security.name.as_ref().unwrap_or(&default_name),
    );
    SECURITY_TYPES.remove(storage, key);
}

/// Checks if the security is in the list of the contract's accepted security types.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `security` - The security to check if it's in the list of acceptable types.
///
/// # Examples
/// ```
/// has_type(deps.storage, Security{category: "category".to_string(), name: None});
/// ```
pub fn has_type(storage: &dyn Storage, security: &Security) -> bool {
    let default_name = String::default();
    let key: (&str, &str) = (
        &security.category,
        &security.name.as_ref().unwrap_or(&default_name),
    );
    SECURITY_TYPES.has(storage, key)
}

/// Obtains all the accepted security types from the contract's store.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `paginate` - Struct containing optional pagination args.
///
/// # Examples
/// ```
/// get_types(deps.storage, Paginate{limit: None, start_after: None});
/// ```
pub fn get_types(
    storage: &dyn Storage,
    paginate: Paginate<Security>,
) -> Result<Vec<Security>, ContractError> {
    let default_name = String::default();
    let start = paginate.start_after.as_ref().map(|security| {
        let key: (&str, &str) = (
            &security.category,
            &security.name.as_ref().unwrap_or(&default_name),
        );
        Bound::exclusive(key)
    });
    let limit = paginate
        .limit
        .unwrap_or(Uint64::new(DEFAULT_SECURITY_TYPES_LIMIT))
        .min(Uint64::new(MAX_SECURITY_TYPES_LIMIT))
        .u64() as usize;
    let keys: Result<Vec<Security>, ContractError> = SECURITY_TYPES
        .keys(storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .map(|result| result.map_err(ContractError::Std))
        .try_fold(Vec::new(), |mut vec, result| {
            vec.push(Security::from(result?));
            Ok(vec)
        });
    keys
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint64;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::{Paginate, Security},
        storage::security::{add_type, get_types, has_type, remove_type},
    };

    #[test]
    fn test_empty_get_types() {
        let deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<Security> = vec![];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_one_item() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let expected: Vec<Security> = vec![Security::new("security1")];
        for security in &expected {
            add_type(deps.as_mut().storage, security).unwrap();
        }
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_multi_item() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let expected: Vec<Security> = vec![Security::new("security1"), Security::new("security2")];
        for security in &expected {
            add_type(deps.as_mut().storage, security).unwrap();
        }
        let types = get_types(&deps.storage, paginate).unwrap();

        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_multi_item_with_name() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let expected: Vec<Security> = vec![
            Security::new("security1"),
            Security::new("security2"),
            Security::new_with_name("security2", "name"),
        ];
        for security in &expected {
            add_type(deps.as_mut().storage, security).unwrap();
        }
        let types = get_types(&deps.storage, paginate).unwrap();

        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_duplicate_entry() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let add: Vec<Security> = vec![
            Security::new("security1"),
            Security::new("security2"),
            Security::new("security2"),
        ];
        let expected: Vec<Security> = vec![Security::new("security1"), Security::new("security2")];
        for security in &add {
            add_type(deps.as_mut().storage, security).unwrap();
        }
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_limit() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: Some(Uint64::new(1)),
            start_after: None,
        };
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        add_type(deps.as_mut().storage, &Security::new("security2")).unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<Security> = vec![Security::new("security1")];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_start_after() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: Some(Security::new("security1")),
        };
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        add_type(deps.as_mut().storage, &Security::new("security2")).unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<Security> = vec![Security::new("security2")];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_default_limit() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let mut expected: Vec<Security> = vec![];
        for i in 1..11 {
            let padded = format!("security{:02}", i);
            let security = Security::new(&padded);
            add_type(deps.as_mut().storage, &security).unwrap();
            expected.push(security);
        }
        add_type(deps.as_mut().storage, &Security::new("security11")).unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();

        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_max_limit() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: Some(Uint64::new(101)),
            start_after: None,
        };
        let mut expected: Vec<Security> = vec![];
        for i in 1..101 {
            let padded = format!("security{:03}", i);
            let security = Security::new(&padded);
            add_type(deps.as_mut().storage, &security).unwrap();
            expected.push(security);
        }
        add_type(deps.as_mut().storage, &Security::new("security101")).unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();

        assert_eq!(expected, types);
    }

    #[test]
    fn test_has_type_is_false_on_empty() {
        let deps = mock_provenance_dependencies();
        let value = has_type(&deps.storage, &Security::new("security1"));
        let expected = false;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_type_is_false_when_missing() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        let value = has_type(&deps.storage, &Security::new("security2"));
        let expected = false;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_type_success() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        let value = has_type(&deps.storage, &Security::new("security1"));
        let expected = true;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_remove_type_empty() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        remove_type(deps.as_mut().storage, &Security::new("security1"));
        let expected: Vec<Security> = vec![];
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_type_invalid() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        remove_type(deps.as_mut().storage, &Security::new("security2"));
        let expected: Vec<Security> = vec![Security::new("security1")];
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_type_single() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        remove_type(deps.as_mut().storage, &Security::new("security1"));
        let expected: Vec<Security> = vec![];
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_type_multiple() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        add_type(deps.as_mut().storage, &Security::new("security2")).unwrap();
        add_type(deps.as_mut().storage, &Security::new("security3")).unwrap();

        remove_type(deps.as_mut().storage, &Security::new("security1"));
        let expected: Vec<Security> = vec![Security::new("security2"), Security::new("security3")];
        let types = get_types(&deps.storage, paginate.clone()).unwrap();
        assert_eq!(expected, types);

        remove_type(deps.as_mut().storage, &Security::new("security2"));
        let expected: Vec<Security> = vec![Security::new("security3")];
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_with_name() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        add_type(deps.as_mut().storage, &Security::new("security1")).unwrap();
        add_type(
            deps.as_mut().storage,
            &Security::new_with_name("security1", "name"),
        )
        .unwrap();

        remove_type(deps.as_mut().storage, &Security::new("security1"));
        let expected: Vec<Security> = vec![Security::new_with_name("security1", "name")];
        let types = get_types(&deps.storage, paginate.clone()).unwrap();
        assert_eq!(expected, types);

        remove_type(
            deps.as_mut().storage,
            &Security::new_with_name("security1", "name"),
        );
        let expected: Vec<Security> = vec![];
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }
}
