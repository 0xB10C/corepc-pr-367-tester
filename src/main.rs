fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_via_client() {
        use corepc_client::types::v29::AddrManInfoNetwork;
        let json_data = r#"
        {
            "new": 5,
            "tried": 10,
            "total": 15,
            "this_is_an_extra_field_not_included_in_the_types": 0
        }
        "#;

        let expected = AddrManInfoNetwork {
            new: 5,
            tried: 10,
            total: 15,
        };

        let parsed: AddrManInfoNetwork = serde_json::from_str(json_data)
            .expect("Failed to deserialize JSON into AddrManInfoNetwork");

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_types_direct() {
        use corepc_types::v29::AddrManInfoNetwork;
        let json_data = r#"
        {
            "new": 5,
            "tried": 10,
            "total": 15,
            "this_is_an_extra_field_not_included_in_the_types": 0
        }
        "#;

        let expected = AddrManInfoNetwork {
            new: 5,
            tried: 10,
            total: 15,
        };

        let parsed: AddrManInfoNetwork = serde_json::from_str(json_data)
            .expect("Failed to deserialize JSON into AddrManInfoNetwork");

        assert_eq!(parsed, expected);
    }
}
