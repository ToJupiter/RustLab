pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut return_vector = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            return_vector.push(line.trim());
        }
    }

    return return_vector;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut return_vector_search = Vec::new();

    let lowercase_query: &str = &query.to_lowercase()[..];

    for line in contents.lines() {
        if line.to_lowercase().contains(lowercase_query) {
            return_vector_search.push(line.trim());
        }
    }

    return return_vector_search;
}