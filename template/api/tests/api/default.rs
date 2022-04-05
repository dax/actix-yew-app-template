use crate::helpers::app_address;
use ::{{crate_name}}::Object;
use rstest::*;

mod get_object {
    use super::*;

    #[rstest]
    #[tokio::test]
    async fn test_hello(app_address: &str) {
        let object: Object = reqwest::Client::new()
            .get(&format!("{}/hello", &app_address))
            .send()
            .await
            .expect("Failed to execute request")
            .json()
            .await
            .expect("Cannot parse JSON result");

        assert_eq!(object.name, "test");
    }
}
