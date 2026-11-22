use std::time::{Duration, SystemTime};

/// Information about the freshness of a rendered response
#[derive(Debug, Clone, Copy)]
pub struct RenderFreshness {
    /// The age of the rendered response
    age: u64,
    /// The maximum age of the rendered response
    max_age: Option<u64>,
    /// The time the response was rendered
    timestamp: SystemTime,
}

impl RenderFreshness {
    /// Create new freshness information
    pub(crate) fn new(age: u64, max_age: u64, timestamp: SystemTime) -> Self {
        Self {
            age,
            max_age: Some(max_age),
            timestamp,
        }
    }

    /// Create new freshness information with only the age
    pub(crate) fn new_age(age: u64, timestamp: SystemTime) -> Self {
        Self {
            age,
            max_age: None,
            timestamp,
        }
    }

    /// Create new freshness information from a timestamp
    pub(crate) fn created_at(timestamp: SystemTime, max_age: Option<Duration>) -> Self {
        Self {
            age: signed_secs(timestamp, SystemTime::now()).unsigned_abs(),
            max_age: max_age.map(|d| d.as_secs()),
            timestamp,
        }
    }

    /// Create new freshness information at the current time
    pub fn now(max_age: Option<Duration>) -> Self {
        Self {
            age: 0,
            max_age: max_age.map(|d| d.as_secs()),
            timestamp: SystemTime::now(),
        }
    }

    /// Get the age of the rendered response in seconds
    pub fn age(&self) -> u64 {
        self.age
    }

    /// Get the maximum age of the rendered response in seconds
    pub fn max_age(&self) -> Option<u64> {
        self.max_age
    }

    /// Get the time the response was rendered
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    /// Write the freshness to the response headers.
    pub fn write(&self, headers: &mut http::HeaderMap<http::HeaderValue>) {
        let age = self.age();
        headers.insert(http::header::AGE, age.into());
        if let Some(max_age) = self.max_age() {
            headers.insert(
                http::header::CACHE_CONTROL,
                http::HeaderValue::from_str(&format!("max-age={}", max_age)).unwrap(),
            );
        }
    }
}

/// `a - b` in whole seconds, truncated toward zero: chrono's `signed_duration_since(..).num_seconds()`.
pub(crate) fn signed_secs(a: SystemTime, b: SystemTime) -> i64 {
    match a.duration_since(b) {
        Ok(ahead) => ahead.as_secs() as i64,
        Err(behind) => -(behind.duration().as_secs() as i64),
    }
}
