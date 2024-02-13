use cosmwasm_std::{Addr, Storage, Uint64};
use cw_storage_plus::{Bound, Map};

use crate::core::{
    aliases::AssetTag,
    constants::{ASSET_TAG_KEY, DEFAULT_WITH_TAG_LIMIT, MAX_WITH_TAG_LIMIT, TAG_TO_ASSET_KEY},
    error::ContractError,
    msg::Paginate,
};

pub const ASSET_TO_TAG: Map<&Addr, AssetTag> = Map::new(ASSET_TAG_KEY);
pub const TAG_TO_ASSET: Map<(AssetTag, &Addr), ()> = Map::new(TAG_TO_ASSET_KEY);

/// Attempts to get the asset's tag that is stored within the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to retrieve the tag of.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// get_tag(deps.storage, &addr)?;
/// `
pub fn get_tag(storage: &dyn Storage, asset_addr: &Addr) -> Result<String, ContractError> {
    Ok(ASSET_TO_TAG.load(storage, asset_addr)?)
}

/// Attempts to get all assets that have the specified tag.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The tag to do a lookup by.
/// * `paginate` - Struct containing optional pagination args.
///
/// # Examples
/// ```
/// with_tag(deps.storage, "tag", Paginate{limit: None, start_after: None})?;
/// `
pub fn with_tag(
    storage: &dyn Storage,
    tag: &str,
    paginate: Paginate<Addr>,
) -> Result<Vec<Addr>, ContractError> {
    let start = paginate.start_after.as_ref().map(Bound::exclusive);
    let limit = paginate
        .limit
        .unwrap_or(Uint64::new(DEFAULT_WITH_TAG_LIMIT))
        .min(Uint64::new(MAX_WITH_TAG_LIMIT))
        .u64() as usize;
    let assets: Result<Vec<Addr>, ContractError> = TAG_TO_ASSET
        .prefix(tag.to_string())
        .keys(storage, start, None, cosmwasm_std::Order::Ascending)
        .map(|result| result.map_err(ContractError::Std))
        .take(limit)
        .collect();
    assets
}

/// Attempts to check if any assets has the supplied tag.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The tag to look for.
///
/// # Examples
/// ```
/// has_tag(deps.storage, "tag")?;
/// `
pub fn has_tag(storage: &dyn Storage, tag: &str) -> bool {
    let tag_is_used = !TAG_TO_ASSET.prefix(tag.to_string()).is_empty(storage);
    tag_is_used
}

/// Attempts to set the asset's tag in the contract's storage.
/// An entry will be put into ASSET_TO_TAG and TAG_TO_ASSET.
/// The previous entry in TAG_TO_ASSET will also be removed.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to tag.
/// * `tag` - The label to add to the asset.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// set_tag(deps.as_mut().storage, &addr, "tag")?;
/// `
pub fn set_tag(
    storage: &mut dyn Storage,
    asset_addr: &Addr,
    tag: &str,
) -> Result<(), ContractError> {
    remove_tag(storage, asset_addr);
    ASSET_TO_TAG.save(storage, asset_addr, &tag.to_string())?;
    Ok(TAG_TO_ASSET.save(storage, (tag.to_string(), asset_addr), &())?)
}

/// Removes the asset's tag from the contract's storage.
/// An entry will be removed from ASSET_TO_TAG and TAG_TO_ASSET.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to tag.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// remove_tag(deps.as_mut().storage, &addr);
/// `
pub fn remove_tag(storage: &mut dyn Storage, asset_addr: &Addr) {
    let tag = get_tag(storage, asset_addr);
    ASSET_TO_TAG.remove(storage, asset_addr);
    if let Ok(tag_to_remove) = tag {
        TAG_TO_ASSET.remove(storage, (tag_to_remove, asset_addr));
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint64};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::Paginate,
        storage::asset::{get_tag, ASSET_TO_TAG, TAG_TO_ASSET},
    };

    use super::{has_tag, remove_tag, set_tag, with_tag};

    #[test]
    fn test_get_tag_empty() {
        let deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let tag = get_tag(&deps.storage, &asset_addr);
        tag.expect_err("should throw an error when asset is missing");
    }

    #[test]
    fn test_has_tag_missing() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        let value = has_tag(&deps.storage, "tag2");
        let expected = false;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_has_tag_success() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        let value = has_tag(&deps.storage, "tag1");
        let expected = true;
        assert_eq!(expected, value);
    }

    #[test]
    fn test_set_tag_single() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        let loaded_tag = ASSET_TO_TAG
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_TAG");
        assert_eq!(loaded_tag, tag.to_string());
        TAG_TO_ASSET
            .load(&deps.storage, (tag.to_string(), &asset_addr))
            .expect("should have entry in TAG_TO_ASSET")
    }

    #[test]
    fn test_set_tag_duplicate() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let tag = "tag1";
        let tag2 = "tag2";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr, tag2).expect("should be successful");

        let loaded_tag = ASSET_TO_TAG
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_TAG");
        assert_eq!(loaded_tag, tag2.to_string());

        TAG_TO_ASSET
            .load(&deps.storage, (tag.to_string(), &asset_addr))
            .expect_err("should remove original entry in TAG_TO_ASSET");
        TAG_TO_ASSET
            .load(&deps.storage, (tag2.to_string(), &asset_addr))
            .expect("should have latest entry in TAG_TO_ASSET");
    }

    #[test]
    fn test_set_tag_multiple() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let tag = "tag1";
        let tag2 = "tag2";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr2, tag2).expect("should be successful");

        let loaded_tag = ASSET_TO_TAG
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_TAG");
        assert_eq!(loaded_tag, tag.to_string());

        let loaded_tag2 = ASSET_TO_TAG
            .load(&deps.storage, &asset_addr2)
            .expect("should have both entries in ASSET_TO_TAG");
        assert_eq!(loaded_tag2, tag2.to_string());

        TAG_TO_ASSET
            .load(&deps.storage, (tag.to_string(), &asset_addr))
            .expect("should have entry in TAG_TO_ASSET");
        TAG_TO_ASSET
            .load(&deps.storage, (tag2.to_string(), &asset_addr2))
            .expect("should have both entries in TAG_TO_ASSET");
    }

    #[test]
    fn test_remove_invalid() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        remove_tag(deps.as_mut().storage, &asset_addr2);

        let loaded_tag = ASSET_TO_TAG
            .load(&deps.storage, &asset_addr)
            .expect("should have entry in ASSET_TO_TAG");
        assert_eq!(loaded_tag, tag.to_string());
        TAG_TO_ASSET
            .load(&deps.storage, (tag.to_string(), &asset_addr))
            .expect("should have entry in TAG_TO_ASSET");
    }

    #[test]
    fn test_remove_single() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        remove_tag(deps.as_mut().storage, &asset_addr);

        ASSET_TO_TAG
            .load(&deps.storage, &asset_addr)
            .expect_err("should have no entry in ASSET_TO_TAG");
        TAG_TO_ASSET
            .load(&deps.storage, (tag.to_string(), &asset_addr))
            .expect_err("should have no entry in TAG_TO_ASSET");
    }

    #[test]
    fn test_remove_multiple() {
        let mut deps = mock_provenance_dependencies();
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let tag = "tag1";
        let tag2 = "tag2";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr2, tag2).expect("should be successful");
        remove_tag(deps.as_mut().storage, &asset_addr);
        remove_tag(deps.as_mut().storage, &asset_addr2);

        ASSET_TO_TAG
            .load(&deps.storage, &asset_addr)
            .expect_err("should not have entry in ASSET_TO_TAG");
        ASSET_TO_TAG
            .load(&deps.storage, &asset_addr2)
            .expect_err("should remove both entries from ASSET_TO_TAG");

        TAG_TO_ASSET
            .load(&deps.storage, (tag.to_string(), &asset_addr))
            .expect_err("should not have entry in TAG_TO_ASSET");
        TAG_TO_ASSET
            .load(&deps.storage, (tag2.to_string(), &asset_addr2))
            .expect_err("should remove both entries frrom TAG_TO_ASSET");
    }

    #[test]
    fn test_with_tag_empty() {
        let deps = mock_provenance_dependencies();
        let expected: Vec<Addr> = vec![];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let tags =
            with_tag(&deps.storage, "tag1", paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
    }

    #[test]
    fn test_with_tag_one_tag() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");

        let tags =
            with_tag(&deps.storage, "tag1", paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
    }

    #[test]
    fn test_with_tag_multi_asset_same_tag() {
        let mut deps = mock_provenance_dependencies();
        let expected = vec![Addr::unchecked("test"), Addr::unchecked("test2")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr2, tag).expect("should be successful");

        let tags = with_tag(&deps.storage, tag, paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
    }

    #[test]
    fn test_with_tag_multi_asset_different_tag() {
        let mut deps = mock_provenance_dependencies();
        let expected1 = vec![Addr::unchecked("test")];
        let expected2 = vec![Addr::unchecked("test2")];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let tag = "tag1";
        let tag2 = "tag2";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr2, tag2).expect("should be successful");

        let tags = with_tag(&deps.storage, tag, paginate.clone())
            .expect("should successfully obtain tags");
        assert_eq!(expected1, tags);

        let tags = with_tag(&deps.storage, tag2, paginate.clone())
            .expect("should successfully obtain tags");
        assert_eq!(expected2, tags);
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
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr2, tag).expect("should be successful");

        let tags = with_tag(&deps.storage, tag, paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
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
        let tag = "tag1";

        set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        set_tag(deps.as_mut().storage, &asset_addr2, tag).expect("should be successful");

        let tags = with_tag(&deps.storage, tag, paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
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
        let tag = "tag1";
        let asset_addr11 = Addr::unchecked("test11");
        for asset_addr in &expected {
            set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        }
        set_tag(deps.as_mut().storage, &asset_addr11, tag).expect("should be successful");

        let tags = with_tag(&deps.storage, tag, paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
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
        let tag = "tag1";
        let remainder = Addr::unchecked("test101");
        for asset_addr in &expected {
            set_tag(deps.as_mut().storage, &asset_addr, tag).expect("should be successful");
        }
        set_tag(deps.as_mut().storage, &remainder, tag).expect("should be successful");

        let tags = with_tag(&deps.storage, tag, paginate).expect("should successfully obtain tags");
        assert_eq!(expected, tags);
    }
}
