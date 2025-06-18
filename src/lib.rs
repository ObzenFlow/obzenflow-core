//! # ObzenFlow
//!
//! Finding zen in the chaos of distributed systems.
//!
//! ## Coming Soon
//!
//! - Event sourcing with perfect consistency
//! - Stream processing that never loses data
//! - Finding beauty in the chaos of production systems
//!
//! Visit [obzenflow.dev](https://obzenflow.dev) for more information.

#![doc(html_logo_url = "https://obzenflow.dev/logo.png")]
#![doc(html_favicon_url = "https://obzenflow.dev/favicon.ico")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// The core ObzenFlow framework
///
/// Currently in active development. This is a placeholder to reserve the crate name
/// while we prepare for the initial release.
pub struct ObzenFlow {
    _private: (),
}

impl ObzenFlow {
    /// Creates a new ObzenFlow instance (placeholder)
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for ObzenFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _flow = ObzenFlow::new();
        assert_eq!(2 + 2, 4);
    }
}
