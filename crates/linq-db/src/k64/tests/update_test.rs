use super::mock_data::TEST_DATA;
use crate::k64::update::*;
use serde_json;
#[test]
fn test_parse() {
    let update: serde_json::Result<DashboardUpdate> =
        serde_json::from_str::<DashboardUpdate>(TEST_DATA);
    assert_eq!(update.is_ok(), true);
}
