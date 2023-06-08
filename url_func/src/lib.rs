use gandiva_rust_udf_macro::udf;
use gandiva_rust_udf_macro::context_fns;

context_fns!();

#[udf(needs_context = true)]
pub fn protocol(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.scheme().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf(needs_context = true)]
pub fn domain(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.domain().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf(needs_context = true)]
pub fn domain_without_www(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.domain().unwrap().trim_start_matches("www.").to_string(),
        Err(_) => String::from(""),
    }
}

#[udf(needs_context = true)]
pub fn top_level_domain(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.domain().unwrap().split('.').last().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}


#[udf(needs_context = true)]
pub fn port(url: &str) -> i32 {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => match u.port() {
            Some(p) => p as i32,
            None => 0,
        },
        Err(_) => 0,
    }
}


// path without query string
#[udf(needs_context = true)]
pub fn path(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.path().to_string(),
        Err(_) => String::from(""),
    }
}

// path withquery string
#[udf(needs_context = true)]
pub fn path_full(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.path().to_string() + "?" + u.query().unwrap_or(""),
        Err(_) => String::from(""),
    }
}


#[udf(needs_context = true)]
pub fn query_string(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.query().unwrap_or("").to_string(),
        Err(_) => String::from(""),
    }
}

#[udf(needs_context = true)]
pub fn fragment(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.fragment().unwrap_or("").to_string(),
        Err(_) => String::from(""),
    }
}

#[udf(needs_context = true)]
pub fn netloc_username(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.username().to_string(),
        Err(_) => String::from(""),
    }
}

#[udf(needs_context = true)]
pub fn netloc_password(url: &str) -> String {
    let url = url::Url::parse(url);
    match url {
        Ok(u) => u.password().unwrap_or("").to_string(),
        Err(_) => String::from(""),
    }
}

// Extracts network locality (username:password@host:port) from a URL.
#[udf(needs_context = true)]
pub fn netloc(url: &str) -> String {
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

#[udf(needs_context = true)]
pub fn is_valid_url(url: &str) -> bool {
    let url = url::Url::parse(url);
    match url {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Removes no more than one 'www.' from the beginning of the URL's domain, if present.
// https://www.example.com:8080/foo/bar?baz=qux#quux ==> https://example.com:8080/foo/bar?baz=qux#quux
#[udf(needs_context = true)]
pub fn cut_www(url: &str) -> String {
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

#[udf(needs_context = true)]
pub fn cut_query_string(url: &str) -> String {
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

#[udf(needs_context = true)]
pub fn cut_query_string_and_fragment(url: &str) -> String {
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
        let result = protocol("https://www.example.com");
        assert_eq!(result, "https");
    }

    #[test]
    fn test_domain() {
        let result = domain("https://www.example.com");
        assert_eq!(result, "www.example.com");
    }

    #[test]
    fn test_domain_without_www() {
        let result = domain_without_www("https://www.example.com");
        assert_eq!(result, "example.com");
    }

    #[test]
    fn test_top_level_domain() {
        let result = top_level_domain("https://www.example.com");
        assert_eq!(result, "com");
    }

    #[test]
    fn test_port() {
        let result = port("https://www.example.com:8080");
        assert_eq!(result, 8080);
    }

    #[test]
    fn test_path() {
        let result = path("https://www.example.com:8080/foo/bar");
        assert_eq!(result, "/foo/bar");
    }

    #[test]
    fn test_path_full() {
        let result = path_full("https://www.example.com:8080/foo/bar?baz=qux");
        assert_eq!(result, "/foo/bar?baz=qux");
    }

    #[test]
    fn test_query_string() {
        let result = query_string("https://www.example.com:8080/foo/bar?baz=qux");
        assert_eq!(result, "baz=qux");
    }

    #[test]
    fn test_fragment() {
        let result = fragment("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "quux");
    }

    #[test]
    fn test_netloc_username() {
        let result = netloc_username("https://me:my_pass@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "me");
    }

    #[test]
    fn test_netloc_password() {
        let result = netloc_password("https://me:my_pass@localhost:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "my_pass");
    }

    #[test]
    fn test_netloc() {
        let result = netloc("https://me:my_pass@www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "me:my_pass@www.example.com:8080");
    }

    #[test]
    fn test_is_valid_url() {
        let result = is_valid_url("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, true);
    }

    #[test]
    fn test_cut_www() {
        let result = cut_www("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "https://example.com:8080/foo/bar?baz=qux#quux");
    }

    #[test]
    fn test_cut_query_string() {
        let result = cut_query_string("https://www.example.com:8080/foo/bar?baz=qux");
        assert_eq!(result, "https://www.example.com:8080/foo/bar");
    }

    #[test]
    fn test_cut_query_string_and_fragment() {
        let result = cut_query_string_and_fragment("https://www.example.com:8080/foo/bar?baz=qux#quux");
        assert_eq!(result, "https://www.example.com:8080/foo/bar");
    }
}
