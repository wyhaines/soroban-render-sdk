//! Router for path-based routing in Soroban Render contracts.
//!
//! Provides path matching utilities and a declarative router inspired by
//! gno.land's mux router, adapted for Soroban's no_std environment.
//!
//! # Pattern Types
//!
//! - Static segments: `/tasks` - exact match
//! - Named parameters: `/users/{id}` - captures segment as variable
//! - Wildcards: `/files/*` - captures remaining path
//!
//! # Example
//!
//! ```rust,ignore
//! use soroban_render_sdk::router::{Router, Request};
//!
//! pub fn render(env: Env, path: Option<String>, viewer: Option<Address>) -> Bytes {
//!     Router::new(&env, path)
//!         .handle(b"/", |_| render_home(&env))
//!         .or_handle(b"/task/{id}", |req| {
//!             let id = req.get_var_u32(b"id").unwrap_or(0);
//!             render_task(&env, id)
//!         })
//!         .or_default(|_| render_home(&env))
//! }
//! ```

use crate::bytes::string_to_bytes;
use soroban_sdk::{Bytes, Env, String};

// ============================================================================
// Path Utilities
// ============================================================================

/// Convert an `Option<String>` path to Bytes, defaulting to "/" if None.
pub fn path_to_bytes(env: &Env, path: &Option<String>) -> Bytes {
    match path {
        Some(p) => string_to_bytes(env, p),
        None => Bytes::from_slice(env, b"/"),
    }
}

/// Split a full path into (path_without_query, query_string).
///
/// For `/create?community=5`, returns (`/create`, Some(`community=5`)).
/// For `/create`, returns (`/create`, None).
pub fn split_path_and_query(env: &Env, full_path: &Bytes) -> (Bytes, Option<Bytes>) {
    let mut path = Bytes::new(env);
    let mut query = Bytes::new(env);
    let mut in_query = false;

    for i in 0..full_path.len() {
        if let Some(b) = full_path.get(i) {
            if b == b'?' && !in_query {
                in_query = true;
                continue; // Skip the '?' itself
            }
            if in_query {
                query.push_back(b);
            } else {
                path.push_back(b);
            }
        }
    }

    // Default to "/" if path is empty
    if path.is_empty() {
        path = Bytes::from_slice(env, b"/");
    }

    let query_opt = if query.is_empty() { None } else { Some(query) };
    (path, query_opt)
}

/// Check if a path exactly equals a route pattern.
///
/// Only works for simple static routes without parameters.
pub fn path_eq(path: &Bytes, route: &[u8]) -> bool {
    if path.len() != route.len() as u32 {
        return false;
    }
    for (i, &b) in route.iter().enumerate() {
        if path.get(i as u32) != Some(b) {
            return false;
        }
    }
    true
}

/// Check if a path starts with a given prefix.
pub fn path_starts_with(path: &Bytes, prefix: &[u8]) -> bool {
    if path.len() < prefix.len() as u32 {
        return false;
    }
    for (i, &b) in prefix.iter().enumerate() {
        if path.get(i as u32) != Some(b) {
            return false;
        }
    }
    true
}

/// Extract the suffix of a path after a prefix.
///
/// Returns the remaining bytes after the prefix.
pub fn path_suffix(env: &Env, path: &Bytes, prefix: &[u8]) -> Bytes {
    let prefix_len = prefix.len() as u32;
    if path.len() <= prefix_len {
        return Bytes::new(env);
    }
    let mut result = Bytes::new(env);
    for i in prefix_len..path.len() {
        if let Some(b) = path.get(i) {
            result.push_back(b);
        }
    }
    result
}

/// Parse a numeric ID from a path with a given prefix.
///
/// For example, given path "/task/123" and prefix "/task/", returns Some(123).
pub fn parse_id(path: &Bytes, prefix: &[u8]) -> Option<u32> {
    let prefix_len = prefix.len() as u32;

    // Check prefix matches
    if path.len() <= prefix_len {
        return None;
    }
    for (i, &b) in prefix.iter().enumerate() {
        if path.get(i as u32) != Some(b) {
            return None;
        }
    }

    // Parse numeric ID from remaining bytes
    let mut result: u32 = 0;
    let mut has_digit = false;

    for i in prefix_len..path.len() {
        if let Some(b) = path.get(i) {
            if b.is_ascii_digit() {
                has_digit = true;
                result = result * 10 + (b - b'0') as u32;
            } else if b == b'/' {
                // Stop at next path segment
                break;
            } else {
                // Non-numeric character
                return None;
            }
        }
    }

    if has_digit { Some(result) } else { None }
}

// ============================================================================
// Request
// ============================================================================

/// A request context containing path information and the matched pattern.
///
/// Used to extract path parameters and query parameters within route handlers.
pub struct Request<'a> {
    env: &'a Env,
    path: Bytes,
    query: Option<Bytes>,
    handler_pattern: &'a [u8],
}

impl<'a> Request<'a> {
    /// Create a new request.
    pub fn new(env: &'a Env, path: Bytes, handler_pattern: &'a [u8]) -> Self {
        Self {
            env,
            path,
            query: None,
            handler_pattern,
        }
    }

    /// Create a new request with query string.
    pub fn with_query(
        env: &'a Env,
        path: Bytes,
        query: Option<Bytes>,
        handler_pattern: &'a [u8],
    ) -> Self {
        Self {
            env,
            path,
            query,
            handler_pattern,
        }
    }

    /// Get the path (without query string).
    pub fn path(&self) -> &Bytes {
        &self.path
    }

    /// Get the raw query string (everything after `?`).
    ///
    /// For path `/create?community=5&foo=bar`, returns Some(`community=5&foo=bar`).
    pub fn raw_query(&self) -> Option<&Bytes> {
        self.query.as_ref()
    }

    /// Get a query parameter value by key.
    ///
    /// For path `/create?community=5&foo=bar`:
    /// - `get_query_param(b"community")` returns Some(`5`)
    /// - `get_query_param(b"foo")` returns Some(`bar`)
    /// - `get_query_param(b"missing")` returns None
    pub fn get_query_param(&self, key: &[u8]) -> Option<Bytes> {
        let query = self.query.as_ref()?;

        // Parse key=value pairs separated by &
        let mut current_key = Bytes::new(self.env);
        let mut current_value = Bytes::new(self.env);
        let mut in_value = false;

        for i in 0..query.len() {
            if let Some(b) = query.get(i) {
                if b == b'=' && !in_value {
                    in_value = true;
                } else if b == b'&' {
                    // Check if current key matches
                    if bytes_eq_slice(&current_key, key) {
                        return Some(current_value);
                    }
                    // Reset for next pair
                    current_key = Bytes::new(self.env);
                    current_value = Bytes::new(self.env);
                    in_value = false;
                } else if in_value {
                    current_value.push_back(b);
                } else {
                    current_key.push_back(b);
                }
            }
        }

        // Check final pair
        if bytes_eq_slice(&current_key, key) {
            return Some(current_value);
        }

        None
    }

    /// Get a query parameter as u32.
    ///
    /// For path `/create?community=5`, `get_query_param_u32(b"community")` returns Some(5).
    pub fn get_query_param_u32(&self, key: &[u8]) -> Option<u32> {
        let bytes = self.get_query_param(key)?;
        parse_bytes_as_u32(&bytes)
    }

    /// Get a query parameter as u64.
    pub fn get_query_param_u64(&self, key: &[u8]) -> Option<u64> {
        let bytes = self.get_query_param(key)?;
        parse_bytes_as_u64(&bytes)
    }

    /// Get a named path parameter value.
    ///
    /// For pattern `/users/{id}` and path `/users/123`,
    /// `get_var(b"id")` returns `Some(Bytes("123"))`.
    pub fn get_var(&self, key: &[u8]) -> Option<Bytes> {
        // Split handler pattern and path into segments
        let pattern_segments = split_path(self.env, self.handler_pattern);
        let path_segments = split_path_bytes(self.env, &self.path);

        // Iterate through pattern segments looking for {key}
        for (path_idx, pattern_seg) in pattern_segments.iter().enumerate() {
            let path_idx = path_idx as u32;
            if path_idx >= path_segments.len() {
                break;
            }

            // Check if this is a parameter segment
            if pattern_seg.len() > 2
                && pattern_seg.get(0) == Some(b'{')
                && pattern_seg.get(pattern_seg.len() - 1) == Some(b'}')
            {
                // Extract parameter name
                let mut param_name = Bytes::new(self.env);
                for i in 1..pattern_seg.len() - 1 {
                    if let Some(b) = pattern_seg.get(i) {
                        param_name.push_back(b);
                    }
                }

                // Check if this matches the requested key
                if bytes_eq_slice(&param_name, key)
                    && let Some(path_seg) = path_segments.get(path_idx)
                {
                    return Some(path_seg);
                }
            }

            // Check for wildcard
            if pattern_seg.len() == 1 && pattern_seg.get(0) == Some(b'*') {
                // Return remaining path
                let mut result = Bytes::new(self.env);
                for i in path_idx..path_segments.len() {
                    if let Some(seg) = path_segments.get(i) {
                        if i > path_idx {
                            result.push_back(b'/');
                        }
                        result.append(&seg);
                    }
                }
                if bytes_eq_slice(&Bytes::from_slice(self.env, b"*"), key) {
                    return Some(result);
                }
            }
        }

        None
    }

    /// Get a path parameter as a u32.
    pub fn get_var_u32(&self, key: &[u8]) -> Option<u32> {
        let bytes = self.get_var(key)?;
        parse_bytes_as_u32(&bytes)
    }

    /// Get the wildcard match (everything after *).
    pub fn get_wildcard(&self) -> Option<Bytes> {
        self.get_var(b"*")
    }
}

// ============================================================================
// Router
// ============================================================================

/// A declarative router for path-based routing.
///
/// Uses first-match-wins semantics. Supports static routes, named parameters,
/// and wildcards. Query strings are automatically stripped for pattern matching
/// but remain accessible via `Request::get_query_param()`.
pub struct Router<'a> {
    env: &'a Env,
    path: Bytes,
    query: Option<Bytes>,
}

impl<'a> Router<'a> {
    /// Create a new router from an optional path.
    ///
    /// Query strings (everything after `?`) are automatically stripped for
    /// pattern matching. Use `Request::get_query_param()` to access them.
    pub fn new(env: &'a Env, path: Option<String>) -> Self {
        let full_path = path_to_bytes(env, &path);
        let (path_only, query) = split_path_and_query(env, &full_path);
        Self {
            env,
            path: path_only,
            query,
        }
    }

    /// Create a router from existing Bytes.
    ///
    /// Query strings are automatically stripped for pattern matching.
    pub fn from_bytes(env: &'a Env, full_path: Bytes) -> Self {
        let (path_only, query) = split_path_and_query(env, &full_path);
        Self {
            env,
            path: path_only,
            query,
        }
    }

    /// Handle a route pattern. Returns a RouterResult for chaining.
    pub fn handle<F, T>(self, pattern: &'a [u8], handler: F) -> RouterResult<'a, T>
    where
        F: FnOnce(Request) -> T,
    {
        if pattern_matches(self.env, &self.path, pattern) {
            let req = Request::with_query(self.env, self.path.clone(), self.query.clone(), pattern);
            RouterResult {
                env: self.env,
                path: self.path,
                query: self.query,
                result: Some(handler(req)),
            }
        } else {
            RouterResult {
                env: self.env,
                path: self.path,
                query: self.query,
                result: None,
            }
        }
    }
}

/// Result of a route match attempt. Allows chaining additional routes.
pub struct RouterResult<'a, T> {
    env: &'a Env,
    path: Bytes,
    query: Option<Bytes>,
    result: Option<T>,
}

impl<'a, T> RouterResult<'a, T> {
    /// Try another route if no match yet.
    pub fn or_handle<F>(self, pattern: &'a [u8], handler: F) -> Self
    where
        F: FnOnce(Request) -> T,
    {
        if self.result.is_some() {
            return self;
        }

        if pattern_matches(self.env, &self.path, pattern) {
            let req = Request::with_query(self.env, self.path.clone(), self.query.clone(), pattern);
            RouterResult {
                env: self.env,
                path: self.path,
                query: self.query,
                result: Some(handler(req)),
            }
        } else {
            self
        }
    }

    /// Provide a default handler. Consumes the result.
    pub fn or_default<F>(self, handler: F) -> T
    where
        F: FnOnce(Request) -> T,
    {
        match self.result {
            Some(r) => r,
            None => {
                let req = Request::with_query(self.env, self.path, self.query, b"");
                handler(req)
            }
        }
    }
}

// ============================================================================
// Pattern Matching
// ============================================================================

/// Check if a path matches a pattern.
///
/// Supports:
/// - Static segments: `/tasks`
/// - Named parameters: `/task/{id}`
/// - Wildcards: `/files/*`
fn pattern_matches(env: &Env, path: &Bytes, pattern: &[u8]) -> bool {
    let pattern_segments = split_path(env, pattern);
    let path_segments = split_path_bytes(env, path);

    // Check for wildcard
    let has_wildcard = pattern.contains(&b'*');

    // If no wildcard, lengths must match
    if !has_wildcard && pattern_segments.len() != path_segments.len() {
        return false;
    }

    // If has wildcard, path must have at least as many segments (minus wildcard)
    if has_wildcard && path_segments.len() < pattern_segments.len() - 1 {
        return false;
    }

    // Match each pattern segment
    for (i, pattern_seg) in pattern_segments.iter().enumerate() {
        // Wildcard matches rest
        if pattern_seg.len() == 1 && pattern_seg.get(0) == Some(b'*') {
            return true;
        }

        // Get corresponding path segment
        if i as u32 >= path_segments.len() {
            return false;
        }
        let path_seg = match path_segments.get(i as u32) {
            Some(s) => s,
            None => return false,
        };

        // Parameter matches any segment
        if pattern_seg.len() > 2
            && pattern_seg.get(0) == Some(b'{')
            && pattern_seg.get(pattern_seg.len() - 1) == Some(b'}')
        {
            continue;
        }

        // Static segment must match exactly
        if pattern_seg.len() != path_seg.len() {
            return false;
        }
        for j in 0..pattern_seg.len() {
            if pattern_seg.get(j) != path_seg.get(j) {
                return false;
            }
        }
    }

    true
}

/// Split a path pattern (byte slice) into segments.
fn split_path(env: &Env, path: &[u8]) -> soroban_sdk::Vec<Bytes> {
    split_path_bytes(env, &Bytes::from_slice(env, path))
}

/// Split a path (Bytes) into segments.
fn split_path_bytes(env: &Env, path: &Bytes) -> soroban_sdk::Vec<Bytes> {
    let mut segments = soroban_sdk::Vec::new(env);
    let mut current = Bytes::new(env);

    for i in 0..path.len() {
        if let Some(b) = path.get(i) {
            if b == b'/' {
                if !current.is_empty() {
                    segments.push_back(current);
                    current = Bytes::new(env);
                }
            } else {
                current.push_back(b);
            }
        }
    }

    if !current.is_empty() {
        segments.push_back(current);
    }

    segments
}

/// Compare Bytes to a byte slice.
fn bytes_eq_slice(bytes: &Bytes, slice: &[u8]) -> bool {
    if bytes.len() != slice.len() as u32 {
        return false;
    }
    for (i, &b) in slice.iter().enumerate() {
        if bytes.get(i as u32) != Some(b) {
            return false;
        }
    }
    true
}

/// Parse Bytes as an unsigned integer.
fn parse_bytes_as_uint<T>(bytes: &Bytes) -> Option<T>
where
    T: From<u8> + core::ops::Mul<Output = T> + core::ops::Add<Output = T> + Copy,
{
    if bytes.is_empty() {
        return None;
    }

    let ten = T::from(10);
    let mut result = T::from(0);
    for i in 0..bytes.len() {
        if let Some(b) = bytes.get(i) {
            if b.is_ascii_digit() {
                result = result * ten + T::from(b - b'0');
            } else {
                return None;
            }
        }
    }
    Some(result)
}

/// Parse Bytes as a u32.
fn parse_bytes_as_u32(bytes: &Bytes) -> Option<u32> {
    parse_bytes_as_uint(bytes)
}

/// Parse Bytes as a u64.
fn parse_bytes_as_u64(bytes: &Bytes) -> Option<u64> {
    parse_bytes_as_uint(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_bytes_none() {
        let env = Env::default();
        let bytes = path_to_bytes(&env, &None);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'/'));
    }

    #[test]
    fn test_path_to_bytes_some() {
        let env = Env::default();
        let s = String::from_str(&env, "/tasks");
        let bytes = path_to_bytes(&env, &Some(s));
        assert_eq!(bytes.len(), 6);
    }

    #[test]
    fn test_path_eq() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/tasks");
        assert!(path_eq(&path, b"/tasks"));
        assert!(!path_eq(&path, b"/task"));
        assert!(!path_eq(&path, b"/tasks/"));
    }

    #[test]
    fn test_path_starts_with() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/tasks/123");
        assert!(path_starts_with(&path, b"/tasks"));
        assert!(path_starts_with(&path, b"/tasks/"));
        assert!(!path_starts_with(&path, b"/task/"));
    }

    #[test]
    fn test_path_suffix() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/tasks/123");
        let suffix = path_suffix(&env, &path, b"/tasks/");
        assert_eq!(suffix.len(), 3);
    }

    #[test]
    fn test_parse_id() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/task/123");
        assert_eq!(parse_id(&path, b"/task/"), Some(123));
    }

    #[test]
    fn test_parse_id_zero() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/task/0");
        assert_eq!(parse_id(&path, b"/task/"), Some(0));
    }

    #[test]
    fn test_parse_id_no_match() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/task/abc");
        assert_eq!(parse_id(&path, b"/task/"), None);
    }

    #[test]
    fn test_pattern_matches_static() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/tasks");
        assert!(pattern_matches(&env, &path, b"/tasks"));
        assert!(!pattern_matches(&env, &path, b"/task"));
    }

    #[test]
    fn test_pattern_matches_param() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/task/123");
        assert!(pattern_matches(&env, &path, b"/task/{id}"));
        assert!(!pattern_matches(&env, &path, b"/task"));
    }

    #[test]
    fn test_pattern_matches_wildcard() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/files/a/b/c");
        assert!(pattern_matches(&env, &path, b"/files/*"));
    }

    #[test]
    fn test_pattern_matches_root() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/");
        assert!(pattern_matches(&env, &path, b"/"));
    }

    #[test]
    fn test_request_get_var() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/users/42/posts");
        let req = Request::new(&env, path, b"/users/{id}/posts");
        let id = req.get_var(b"id");
        assert!(id.is_some());
        let id_bytes = id.unwrap();
        assert_eq!(id_bytes.len(), 2);
        assert_eq!(id_bytes.get(0), Some(b'4'));
        assert_eq!(id_bytes.get(1), Some(b'2'));
    }

    #[test]
    fn test_request_get_var_u32() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/task/123");
        let req = Request::new(&env, path, b"/task/{id}");
        assert_eq!(req.get_var_u32(b"id"), Some(123));
    }

    #[test]
    fn test_request_get_wildcard() {
        let env = Env::default();
        let path = Bytes::from_slice(&env, b"/files/a/b/c");
        let req = Request::new(&env, path, b"/files/*");
        let wildcard = req.get_wildcard();
        assert!(wildcard.is_some());
    }

    #[test]
    fn test_router_handle() {
        let env = Env::default();
        let result = Router::new(&env, Some(String::from_str(&env, "/tasks")))
            .handle(b"/tasks", |_| 42u32)
            .or_default(|_| 0u32);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_router_or_handle() {
        let env = Env::default();
        let result = Router::new(&env, Some(String::from_str(&env, "/about")))
            .handle(b"/", |_| 1u32)
            .or_handle(b"/about", |_| 2u32)
            .or_default(|_| 0u32);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_router_default() {
        let env = Env::default();
        let result = Router::new(&env, Some(String::from_str(&env, "/unknown")))
            .handle(b"/", |_| 1u32)
            .or_handle(b"/about", |_| 2u32)
            .or_default(|_| 99u32);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_router_with_param() {
        let env = Env::default();
        let result = Router::new(&env, Some(String::from_str(&env, "/task/42")))
            .handle(b"/task/{id}", |req| req.get_var_u32(b"id").unwrap_or(0))
            .or_default(|_| 0u32);
        assert_eq!(result, 42);
    }

    // ========================================================================
    // Query String Tests
    // ========================================================================

    #[test]
    fn test_split_path_and_query_no_query() {
        let env = Env::default();
        let full = Bytes::from_slice(&env, b"/create");
        let (path, query) = split_path_and_query(&env, &full);
        assert_eq!(path.len(), 7);
        assert!(query.is_none());
    }

    #[test]
    fn test_split_path_and_query_with_query() {
        let env = Env::default();
        let full = Bytes::from_slice(&env, b"/create?community=5");
        let (path, query) = split_path_and_query(&env, &full);
        // Path should be "/create"
        assert_eq!(path.len(), 7);
        assert_eq!(path.get(0), Some(b'/'));
        assert_eq!(path.get(6), Some(b'e'));
        // Query should be "community=5"
        assert!(query.is_some());
        let q = query.unwrap();
        assert_eq!(q.len(), 11);
    }

    #[test]
    fn test_split_path_and_query_multiple_params() {
        let env = Env::default();
        let full = Bytes::from_slice(&env, b"/search?q=hello&page=2&sort=date");
        let (path, query) = split_path_and_query(&env, &full);
        assert_eq!(path.len(), 7); // "/search"
        assert!(query.is_some());
        let q = query.unwrap();
        // "q=hello&page=2&sort=date" = 24 chars
        assert_eq!(q.len(), 24);
    }

    #[test]
    fn test_router_matches_with_query_string() {
        let env = Env::default();
        // This is the key test - /create?community=0 should match /create
        let result = Router::new(&env, Some(String::from_str(&env, "/create?community=0")))
            .handle(b"/create", |_| 42u32)
            .or_default(|_| 0u32);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_router_query_param_accessible() {
        let env = Env::default();
        let result = Router::new(&env, Some(String::from_str(&env, "/create?community=5")))
            .handle(b"/create", |req| {
                req.get_query_param_u32(b"community").unwrap_or(0)
            })
            .or_default(|_| 0u32);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_router_query_param_u64() {
        let env = Env::default();
        let result = Router::new(
            &env,
            Some(String::from_str(&env, "/create?community=12345678901")),
        )
        .handle(b"/create", |req| {
            req.get_query_param_u64(b"community").unwrap_or(0)
        })
        .or_default(|_| 0u64);
        assert_eq!(result, 12345678901u64);
    }

    #[test]
    fn test_router_multiple_query_params() {
        let env = Env::default();
        let result = Router::new(
            &env,
            Some(String::from_str(&env, "/search?q=hello&page=3&sort=date")),
        )
        .handle(b"/search", |req| {
            let page = req.get_query_param_u32(b"page").unwrap_or(1);
            let has_sort = req.get_query_param(b"sort").is_some();
            if has_sort { page * 10 } else { page }
        })
        .or_default(|_| 0u32);
        assert_eq!(result, 30); // page=3, has_sort=true, so 3*10=30
    }

    #[test]
    fn test_router_query_param_missing() {
        let env = Env::default();
        let result = Router::new(&env, Some(String::from_str(&env, "/create?other=value")))
            .handle(b"/create", |req| {
                req.get_query_param_u32(b"community").unwrap_or(999)
            })
            .or_default(|_| 0u32);
        assert_eq!(result, 999); // community param missing, use default
    }

    #[test]
    fn test_router_raw_query() {
        let env = Env::default();
        let result = Router::new(
            &env,
            Some(String::from_str(&env, "/create?foo=bar&baz=qux")),
        )
        .handle(b"/create", |req| {
            req.raw_query().map(|q| q.len()).unwrap_or(0)
        })
        .or_default(|_| 0u32);
        assert_eq!(result, 15); // "foo=bar&baz=qux" = 15 chars
    }

    #[test]
    fn test_router_path_with_param_and_query() {
        let env = Env::default();
        // Test path params work alongside query params
        let result = Router::new(&env, Some(String::from_str(&env, "/task/42?status=done")))
            .handle(b"/task/{id}", |req| {
                let id = req.get_var_u32(b"id").unwrap_or(0);
                let has_status = req.get_query_param(b"status").is_some();
                if has_status { id + 100 } else { id }
            })
            .or_default(|_| 0u32);
        assert_eq!(result, 142); // id=42, has_status=true, so 42+100=142
    }

    // ========================================================================
    // Query String Edge Cases
    // ========================================================================

    #[test]
    fn test_split_path_and_query_empty_query() {
        let env = Env::default();
        // Path with trailing ? but no query params
        let full = Bytes::from_slice(&env, b"/create?");
        let (path, query) = split_path_and_query(&env, &full);
        assert_eq!(path.len(), 7); // "/create"
        assert!(query.is_none()); // Empty query should be None
    }

    #[test]
    fn test_query_param_empty_value() {
        let env = Env::default();
        // Query param with empty value: foo=
        let result = Router::new(&env, Some(String::from_str(&env, "/test?foo=&bar=value")))
            .handle(b"/test", |req| {
                let foo = req.get_query_param(b"foo");
                let bar = req.get_query_param(b"bar");
                // foo should exist but be empty, bar should have value
                let foo_empty = foo.map(|b| b.is_empty()).unwrap_or(false);
                let bar_ok = bar.map(|b| b.len() == 5).unwrap_or(false);
                if foo_empty && bar_ok { 1u32 } else { 0u32 }
            })
            .or_default(|_| 0u32);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_query_param_no_equals() {
        let env = Env::default();
        // Query param without = sign (just key) - treated as key with empty value
        let result = Router::new(&env, Some(String::from_str(&env, "/test?flag&value=123")))
            .handle(b"/test", |req| {
                // "flag" has no = sign, treated as existing with empty value
                let flag = req.get_query_param(b"flag");
                let value = req.get_query_param_u32(b"value");
                // flag should exist but be empty, value should be 123
                let flag_empty = flag.map(|b| b.is_empty()).unwrap_or(false);
                if flag_empty && value == Some(123) {
                    1u32
                } else {
                    0u32
                }
            })
            .or_default(|_| 0u32);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_query_first_param() {
        let env = Env::default();
        // Ensure first param is accessible
        let result = Router::new(
            &env,
            Some(String::from_str(&env, "/test?first=1&second=2&third=3")),
        )
        .handle(b"/test", |req| {
            req.get_query_param_u32(b"first").unwrap_or(0)
        })
        .or_default(|_| 0u32);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_query_last_param() {
        let env = Env::default();
        // Ensure last param is accessible (no trailing &)
        let result = Router::new(
            &env,
            Some(String::from_str(&env, "/test?first=1&second=2&third=3")),
        )
        .handle(b"/test", |req| {
            req.get_query_param_u32(b"third").unwrap_or(0)
        })
        .or_default(|_| 0u32);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_router_from_bytes_with_query() {
        let env = Env::default();
        // Test from_bytes also strips query params
        let full_path = Bytes::from_slice(&env, b"/create?id=5");
        let result = Router::from_bytes(&env, full_path)
            .handle(b"/create", |req| {
                req.get_query_param_u32(b"id").unwrap_or(0)
            })
            .or_default(|_| 0u32);
        assert_eq!(result, 5);
    }
}
