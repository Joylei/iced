//! Ime request

use std::time::Instant;

/// Request IME
#[derive(Debug, Clone, Copy)]
pub enum ImeRequest {
    /// Disable IME
    SetAllowed {
        /// Time for request
        at: Instant,
        /// Allow to show IME window
        allowed: bool,
    },
    /// Set IME window position
    SetPosition {
        /// Time for request
        at: Instant,
        /// IME window position
        position: (i32, i32),
    },
}

impl ImeRequest {
    /// Time for request
    fn time(&self) -> Instant {
        match self {
            ImeRequest::SetAllowed { at, .. } => *at,
            ImeRequest::SetPosition { at, .. } => *at,
        }
    }

    /// merge request
    pub(crate) fn merge(&mut self, other: ImeRequest) {
        if self.time() < other.time() {
            *self = other;
        }
    }

    /// check if  same request
    pub(crate) fn is_same(&self, other: &ImeRequest) -> bool {
        match (self, other) {
            (
                ImeRequest::SetAllowed { allowed: a, .. },
                ImeRequest::SetAllowed { allowed: b, .. },
            ) => a == b,
            (
                ImeRequest::SetPosition { position: a, .. },
                ImeRequest::SetPosition { position: b, .. },
            ) => a == b,
            _ => false,
        }
    }
}
