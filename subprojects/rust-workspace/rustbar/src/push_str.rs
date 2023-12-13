use std::{ffi::c_char, fmt::Display};

/// A mutable String that is guarenteed to always end with a null byte
pub struct PushString {
    inner: String,
}

impl Display for PushString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Default for PushString {
    fn default() -> Self {
        Self::new()
    }
}

impl PushString {
    pub fn new() -> Self {
        Self {
            inner: String::from("\0"),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
        self.inner.push('\0');
    }

    // pub fn push(&mut self, c: char) {
    //     self.inner.pop();
    //     self.inner.push(c);
    //     self.inner.push('\0');
    // }

    pub fn push_str(&mut self, s: &str) {
        self.inner.pop();
        self.inner.push_str(s);
        self.inner.push('\0');
    }

    pub fn pop(&mut self) {
        self.inner.pop();
        self.inner.pop();
        self.inner.push('\0');
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr().cast()
    }
}

impl AsRef<str> for PushString {
    fn as_ref(&self) -> &str {
        &self.inner[..self.inner.len() - 1]
    }
}
