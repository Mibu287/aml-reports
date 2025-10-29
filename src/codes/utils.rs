pub fn search_for_code(code_list: &[(&str, &str)], name: &str) -> String {
    code_list
        .iter()
        .find_map(|(code, status_name)| {
            if status_name.eq_ignore_ascii_case(name) {
                Some(code.to_string())
            } else {
                None
            }
        })
        .unwrap_or_default()
}
