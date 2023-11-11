use std::collections::HashMap;

pub fn opt_attr(key: &str, val: String) -> String {
    if val.is_empty() {
        String::from("")
    } else {
        format!("{}=\"{}\"", key, val)
    }
}

pub fn opt_attrs(map: HashMap<&str, String>) -> String {
    if map.is_empty() {
        String::from("")
    } else {
        let mut attrs = map
            .iter()
            .map(|(key, val)| opt_attr(key, val.to_string()))
            .collect::<Vec<String>>();

        // Output attributes in alpha order.
        attrs.sort_by(|a, b| a.cmp(b));
        attrs.join(" ").trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_opt_attr_with_empty_string() {
        assert_eq!(opt_attr("foo", String::from("")), String::from(""));
    }

    #[test]
    fn test_opt_attr_with_non_empty_string() {
        assert_eq!(
            opt_attr("foo", String::from("baz")),
            String::from("foo=\"baz\"")
        );
    }

    #[test]
    fn test_opt_attr_with_string_with_spaces() {
        assert_eq!(
            opt_attr("foo", String::from("foo bar baz")),
            String::from("foo=\"foo bar baz\"")
        );
    }

    #[test]
    fn test_opt_attrs_with_empty_map() {
        assert_eq!(opt_attrs(HashMap::new()), String::from(""));
    }

    #[test]
    fn test_opt_attrs_with_empty_map_empty_array() {
        assert_eq!(opt_attrs(HashMap::from([])), String::from(""));
    }

    #[test]
    fn test_opt_attrs_with_single_attr_that_is_empty() {
        assert_eq!(
            opt_attrs(HashMap::from([("foo", String::from(""))])),
            String::from("")
        );
    }

    #[test]
    fn test_opt_attrs_with_multiple_attrs_that_are_empty() {
        assert_eq!(
            opt_attrs(HashMap::from([
                ("foo", String::from("")),
                ("baz", String::from("")),
            ])),
            String::from("")
        );
    }

    #[test]
    fn test_opt_attrs_with_single_attribute_tuple() {
        assert_eq!(
            opt_attrs(HashMap::from([("foo", String::from("baz"))])),
            String::from("foo=\"baz\"")
        );
    }

    #[test]
    fn test_opt_attrs_with_multiple_attribute_tuple() {
        let attrs = opt_attrs(HashMap::from([
            ("foo", String::from("baz")),
            ("bar", String::from("fuzz fuzz-baz")),
        ]));

        assert_eq!(attrs, String::from("bar=\"fuzz fuzz-baz\" foo=\"baz\""),);
    }
}
