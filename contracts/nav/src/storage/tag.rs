use cosmwasm_std::Storage;
use cw_storage_plus::Map;

use crate::core::{constants::ASSET_TYPE_KEY, error::ContractError};

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
    return TAG_TYPES.has(storage, tag);
}

/// Obtains all the accepted tag types from the contract's store.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
///
/// # Examples
/// ```
/// get_types(deps.storage);
/// `
pub fn get_types(storage: &dyn Storage) -> Result<Vec<String>, ContractError> {
    let keys: Result<Vec<String>, ContractError> = TAG_TYPES
        .keys(storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|result| result.map_err(|err| ContractError::Std(err)))
        .collect();
    keys
}

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::storage::tag::{add_type, get_types, has_type, remove_type};

    #[test]
    fn test_empty_get_types() {
        let deps = mock_provenance_dependencies();
        let types = get_types(&deps.storage).unwrap();
        let expected: Vec<String> = vec![];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_one_item() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        let expected: Vec<String> = vec!["tag1".to_string()];
        let types = get_types(&deps.storage).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_multi_item() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        let types = get_types(&deps.storage).unwrap();
        let expected: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        assert_eq!(expected, types);
    }

    #[test]
    fn test_add_and_get_duplicate_entry() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        let types = get_types(&deps.storage).unwrap();
        let expected: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
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
        remove_type(deps.as_mut().storage, "tag1");
        let expected: Vec<String> = vec![];
        let types = get_types(&deps.storage).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_type_invalid() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        remove_type(deps.as_mut().storage, "tag2");
        let expected: Vec<String> = vec!["tag1".to_string()];
        let types = get_types(&deps.storage).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_type_single() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        remove_type(deps.as_mut().storage, "tag1");
        let expected: Vec<String> = vec![];
        let types = get_types(&deps.storage).unwrap();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_remove_type_multiple() {
        let mut deps = mock_provenance_dependencies();
        add_type(deps.as_mut().storage, "tag1").unwrap();
        add_type(deps.as_mut().storage, "tag2").unwrap();
        add_type(deps.as_mut().storage, "tag3").unwrap();

        remove_type(deps.as_mut().storage, "tag1");
        let expected: Vec<String> = vec!["tag2".to_string(), "tag3".to_string()];
        let types = get_types(&deps.storage).unwrap();
        assert_eq!(expected, types);

        remove_type(deps.as_mut().storage, "tag2");
        let expected: Vec<String> = vec!["tag3".to_string()];
        let types = get_types(&deps.storage).unwrap();
        assert_eq!(expected, types);
    }
}
