use cosmwasm_std::{Storage, Uint64};
use cw_storage_plus::{Bound, Map};

use crate::core::{
    aliases::AssetTag,
    constants::{ASSET_TYPE_KEY, DEFAULT_TAG_TYPES_LIMIT, MAX_TAG_TYPES_LIMIT},
    error::ContractError,
    msg::Paginate,
};

pub const TAG_TYPES: Map<&str, ()> = Map::new(ASSET_TYPE_KEY);

/// Attempts to add a tag to the list of acceptable types in the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The label to add to the list of acceptable types.
///
/// # Examples
/// ```
/// add_type(deps.as_mut().storage, "tag")?;
/// `
pub fn add_type(storage: &mut dyn Storage, tag: &str) -> Result<(), ContractError> {
    Ok(TAG_TYPES.save(storage, tag, &())?)
}

/// Removes the tag type from the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The label to remove from the list of acceptable types.
///
/// # Examples
/// ```
/// remove_type(deps.as_mut().storage, "tag");
/// `
pub fn remove_type(storage: &mut dyn Storage, tag: &str) {
    TAG_TYPES.remove(storage, tag);
}

/// Checks if the tag is in the list of the contract's accepted tag types.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The label to check if it's in the list of acceptable types.
///
/// # Examples
/// ```
/// has_type(deps.storage, "tag");
/// `
pub fn has_type(storage: &dyn Storage, tag: &str) -> bool {
    TAG_TYPES.has(storage, tag)
}

/// Obtains all the accepted tag types from the contract's store.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `paginate` - Struct containing optional pagination args.
///
/// # Examples
/// ```
/// get_types(deps.storage, Paginate{limit: None, start_after: None});
/// `
pub fn get_types(
    storage: &dyn Storage,
    paginate: Paginate<AssetTag>,
) -> Result<Vec<AssetTag>, ContractError> {
    let start = paginate
        .start_after
        .as_ref()
        .map(|str| Bound::exclusive(str.as_str()));
    let limit = paginate
        .limit
        .unwrap_or(Uint64::new(DEFAULT_TAG_TYPES_LIMIT))
        .min(Uint64::new(MAX_TAG_TYPES_LIMIT))
        .u64() as usize;
    let keys: Result<Vec<AssetTag>, ContractError> = TAG_TYPES
        .keys(storage, start, None, cosmwasm_std::Order::Ascending)
        .map(|result| result.map_err(ContractError::Std))
        .take(limit)
        .collect();
    keys
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint64;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::Paginate,
        storage::tag::{add_type, get_types, has_type, remove_type},
    };

    #[test]
    fn test_empty_get_types() {
        let deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<String> = vec![];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_one_item() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        add_type(deps.as_mut().storage, "tag1").unwrap();
        let expected: Vec<String> = vec!["tag1".to_string()];
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
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_duplicate_entry() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_limit() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: Some(Uint64::new(1)),
            start_after: None,
        };
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<String> = vec!["tag1".to_string()];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_start_after() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: Some("tag1".to_string()),
        };
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();
        let expected: Vec<String> = vec!["tag2".to_string()];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_type_paginate_default_limit() {
        let mut deps = mock_provenance_dependencies();
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let mut expected: Vec<String> = vec![];
        for i in 1..11 {
            let padded = format!("tag{:02}", i);
            add_type(deps.as_mut().storage, &padded).unwrap();
            expected.push(padded);
        }
        add_type(deps.as_mut().storage, "tag11").unwrap();
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
        let mut expected: Vec<String> = vec![];
        for i in 1..101 {
            let padded = format!("tag{:03}", i);
            add_type(deps.as_mut().storage, &padded).unwrap();
            expected.push(padded);
        }
        add_type(deps.as_mut().storage, "tag101").unwrap();
        let types = get_types(&deps.storage, paginate).unwrap();

        assert_eq!(expected, types);
    }

    #[test]
    fn test_has_type_is_false_on_empty() {
        let deps = mock_provenance_dependencies();
        let value = has_type(&deps.storage, "tag1");
        let expected = false;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_type_is_false_when_missing() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        let value = has_type(&deps.storage, "tag2");
        let expected = false;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_type_success() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        let value = has_type(&deps.storage, "tag1");
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
        remove_type(deps.as_mut().storage, "tag1");
        let expected: Vec<String> = vec![];
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
        add_type(deps.as_mut().storage, "tag1").unwrap();
        remove_type(deps.as_mut().storage, "tag2");
        let expected: Vec<String> = vec!["tag1".to_string()];
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
        add_type(deps.as_mut().storage, "tag1").unwrap();
        remove_type(deps.as_mut().storage, "tag1");
        let expected: Vec<String> = vec![];
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
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        add_type(deps.as_mut().storage, "tag3").unwrap();

        remove_type(deps.as_mut().storage, "tag1");
        let expected: Vec<String> = vec!["tag2".to_string(), "tag3".to_string()];
        let types = get_types(&deps.storage, paginate.clone()).unwrap();
        assert_eq!(expected, types);

        remove_type(deps.as_mut().storage, "tag2");
        let expected: Vec<String> = vec!["tag3".to_string()];
        let types = get_types(&deps.storage, paginate).unwrap();
        assert_eq!(expected, types);
    }
}
