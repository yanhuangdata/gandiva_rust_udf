use gandiva_rust_udf_macro::udf;

#[udf]
fn protocol(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.scheme().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn domain(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.domain().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn domain_without_www(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.domain().unwrap().trim_start_matches("www.").to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn top_level_domain(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.domain().unwrap().split('.').last().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn port(url: &str) -> i32 {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => match u.port() {
            Some(p) => p as i32,
            // TODO: may return null
            None => 0,
        },
        Err(_) => 0,
    }
}

// path without query string
#[udf]
fn path(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.path().to_string(),
        Err(_) => String::from(""),
    }
}

// path withquery string
#[udf]
fn path_full(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.path().to_string() + "?" + u.query().unwrap_or(""),
        Err(_) => String::from(""),
    }
}

#[udf]
fn query_string(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.query().unwrap_or("").to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn fragment(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.fragment().unwrap_or("").to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn netloc_username(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.username().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf]
fn netloc_password(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.password().unwrap_or("").to_string(),
        Err(_) => String::from(""),
    }
}

// Extracts network locality (username:password@host:port) from a URL.
#[udf]
fn netloc(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => {
            let mut netloc = String::new();
            if u.username() != "" {
                netloc.push_str(u.username());
                if u.password().unwrap_or("") != "" {
                    netloc.push_str(":");
                    netloc.push_str(u.password().unwrap_or(""));
                }
                netloc.push_str("@");
            }
            netloc.push_str(u.host_str().unwrap_or(""));
            if u.port() != None {
                netloc.push_str(":");
                netloc.push_str(&u.port().unwrap().to_string());
            }
            netloc
        }
        Err(_) => String::from(""),
    }
}

#[udf]
fn is_valid_url(url: &str) -> bool {
    let url = url::Url::parse(url);
    match url {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Removes no more than one 'www.' from the beginning of the URL's domain, if present.
// https://www.example.com:8080/foo/bar?baz=qux#quux ==> https://example.com:8080/foo/bar?baz=qux#quux
#[udf]
fn cut_www(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => {
            let mut domain = u.domain().unwrap().to_string();
            if domain.starts_with("www.") {
                domain = domain.trim_start_matches("www.").to_string();
            }
            let mut new_url = u.clone();
            new_url.set_host(Some(&domain)).unwrap();
            new_url.as_str().to_string()
        }
        Err(_) => String::from(""),
    }
}

#[udf]
fn cut_query_string(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => {
            let mut new_url = u.clone();
            new_url.set_query(None);
            new_url.as_str().to_string()
        }
        Err(_) => String::from(""),
    }
}

#[udf]
fn cut_query_string_and_fragment(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => {
            let mut new_url = u.clone();
            new_url.set_query(None);
            new_url.set_fragment(None);
            new_url.as_str().to_string()
        }
        Err(_) => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol() {
        let mut result = protocol("https://www.example.com");
        assert_eq!(result, "https");

        result = protocol("//www.example.com");
        assert_eq!(result, "");
    }

    #[test]
    fn test_domain() {
        let mut result = domain("https://www.example.com");
        assert_eq!(result, "www.example.com");

        result = domain("www.example.com");
        assert_eq!(result, "");
    }

    #[test]
    fn test_domain_without_www() {
        let mut result = domain_without_www("https://www.example.com");
        assert_eq!(result, "example.com");

        result = domain_without_www("https://example.com");
        assert_eq!(result, "example.com");
    }

    #[test]
    fn test_top_level_domain() {
        let mut result = top_level_domain("https://www.example.com");
        assert_eq!(result, "com");

        result = top_level_domain("https://www.example.com/next");
        assert_eq!(result, "com");

        result = top_level_domain("https://example.com/next");
        assert_eq!(result, "com");

        result = top_level_domain("www.example.com/next");
        assert_eq!(result, "");
    }

    #[test]
    fn test_port() {
        let mut result = port("https://www.example.com:8080");
        assert_eq!(result, 8080);

        result = port("www.example.com:8080");
        assert_eq!(result, 0);

        result = port("https://www.example.com");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_path() {
        let mut result = path("https://www.example.com:8080/foo/bar");
        assert_eq!(result, "/foo/bar");

        result = path("https://www.example.com/foo/bar");
        assert_eq!(result, "/foo/bar");

        // TODO: may return ""
        result = path("www.example.com:8080/foo/bar");
        assert_eq!(result, "8080/foo/bar");
    }

    #[test]
    fn test_path_full() {
        let mut result = path_full("https://www.example.com:8080/foo/bar?baz=qux");
        assert_eq!(result, "/foo/bar?baz=qux");

        // TODO: may return ""
        result = path_full("www.example.com:8080/foo/bar?baz=qux&q=a");
        assert_eq!(result, "8080/foo/bar?baz=qux&q=a");
    }

    #[test]
    fn test_query_string() {
        let mut result = query_string("https://www.example.com:8080/foo/bar?baz=qux");
        assert_eq!(result, "baz=qux");

        result = query_string("www.example.com/foo/bar?baz=qux&q=a");
        assert_eq!(result, "");
    }

    #[test]
    fn test_fragment() {
        let mut result = fragment("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "quux");

        result = fragment("/www.example.com/foo/bar?baz=qux#quux");
        assert_eq!(result, "");
    }

    #[test]
    fn test_netloc_username() {
        let mut result =
            netloc_username("https://me:my_pass@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "me");

        result = netloc_username("https://me:@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "me");

        result = netloc_username("//me:my_pass@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "");
    }

    #[test]
    fn test_netloc_password() {
        let mut result = netloc_password("https://me:my_pass@localhost:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "my_pass");

        result = netloc_password("https://me@localhost:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "");

        result = netloc_password("https://me:@localhost:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "");
    }

    #[test]
    fn test_netloc() {
        let mut result = netloc("https://me:my_pass@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "me:my_pass@www.example.com:8080");

        result = netloc("https://me@www.example.com/foo/bar?baz=qux#quux");
        assert_eq!(result, "me@www.example.com");

        result = netloc("https://me:@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "me@www.example.com:8080");
    }

    #[test]
    fn test_is_valid_url() {
        let mut result = is_valid_url("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, true);

        result = is_valid_url("/www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, false);
    }

    #[test]
    fn test_cut_www() {
        let mut result = cut_www("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "https://example.com:8080/foo/bar?baz=qux#quux");

        result = cut_www("https://example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "https://example.com:8080/foo/bar?baz=qux#quux");

        result = cut_www("https://www.www.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "https://com:8080/foo/bar?baz=qux#quux");
    }

    #[test]
    fn test_cut_query_string() {
        let mut result = cut_query_string("https://www.example.com:8080/foo/bar?baz=qux");
        assert_eq!(result, "https://www.example.com:8080/foo/bar");

        result = cut_query_string("https://www.example.com:8080?baz=qux");
        assert_eq!(result, "https://www.example.com:8080/");

        result = cut_query_string("https://www.example.com?baz=qux");
        assert_eq!(result, "https://www.example.com/");
    }

    #[test]
    fn test_cut_query_string_and_fragment() {
        let mut result =
            cut_query_string_and_fragment("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "https://www.example.com:8080/foo/bar");

        result = cut_query_string_and_fragment("https://www.example.com:8080#quux");
        assert_eq!(result, "https://www.example.com:8080/");

        result = cut_query_string_and_fragment("https://www.example.com#quux");
        assert_eq!(result, "https://www.example.com/");
    }
}
