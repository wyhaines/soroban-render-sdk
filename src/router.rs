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
/// Used to extract path parameters within route handlers.
pub struct Request<'a> {
    env: &'a Env,
    path: Bytes,
    handler_pattern: &'a [u8],
}

impl<'a> Request<'a> {
    /// Create a new request.
    pub fn new(env: &'a Env, path: Bytes, handler_pattern: &'a [u8]) -> Self {
        Self {
            env,
            path,
            handler_pattern,
        }
    }

    /// Get the full path.
    pub fn path(&self) -> &Bytes {
        &self.path
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
/// and wildcards.
pub struct Router<'a> {
    env: &'a Env,
    path: Bytes,
}

impl<'a> Router<'a> {
    /// Create a new router from an optional path.
    pub fn new(env: &'a Env, path: Option<String>) -> Self {
        Self {
            env,
            path: path_to_bytes(env, &path),
        }
    }

    /// Create a router from existing Bytes.
    pub fn from_bytes(env: &'a Env, path: Bytes) -> Self {
        Self { env, path }
    }

    /// Handle a route pattern. Returns a RouterResult for chaining.
    pub fn handle<F, T>(self, pattern: &'a [u8], handler: F) -> RouterResult<'a, T>
    where
        F: FnOnce(Request) -> T,
    {
        if pattern_matches(self.env, &self.path, pattern) {
            let req = Request::new(self.env, self.path.clone(), pattern);
            RouterResult {
                env: self.env,
                path: self.path,
                result: Some(handler(req)),
            }
        } else {
            RouterResult {
                env: self.env,
                path: self.path,
                result: None,
            }
        }
    }
}

/// Result of a route match attempt. Allows chaining additional routes.
pub struct RouterResult<'a, T> {
    env: &'a Env,
    path: Bytes,
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
            let req = Request::new(self.env, self.path.clone(), pattern);
            RouterResult {
                env: self.env,
                path: self.path,
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
                let req = Request::new(self.env, self.path, b"");
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
    let mut segments = soroban_sdk::Vec::new(env);
    let mut current = Bytes::new(env);

    for &b in path {
        if b == b'/' {
            if !current.is_empty() {
                segments.push_back(current);
                current = Bytes::new(env);
            }
        } else {
            current.push_back(b);
        }
    }

    if !current.is_empty() {
        segments.push_back(current);
    }

    segments
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

/// Parse Bytes as a u32.
fn parse_bytes_as_u32(bytes: &Bytes) -> Option<u32> {
    if bytes.is_empty() {
        return None;
    }

    let mut result: u32 = 0;
    for i in 0..bytes.len() {
        if let Some(b) = bytes.get(i) {
            if b.is_ascii_digit() {
                result = result * 10 + (b - b'0') as u32;
            } else {
                return None;
            }
        }
    }
    Some(result)
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
}
