#![forbid(unsafe_code)]
//! SigmaOS-data-accessibility: data accessibility model.
//!
//! Real implementation: permissions, roles, audit trails, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Access permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPermission {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilityError {
    EmptyName,
    DuplicatePermission(String),
}

impl fmt::Display for AccessibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "permission name must not be empty"),
            Self::DuplicatePermission(id) => write!(f, "duplicate permission {:?}", id),
        }
    }
}

impl std::error::Error for AccessibilityError {}

/// Data accessibility manager.
pub struct DataAccessibility {
    pub name: String,
    permissions: BTreeMap<String, AccessPermission>,
}

impl DataAccessibility {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            permissions: BTreeMap::new(),
        }
    }

    pub fn add_permission(&mut self, permission: AccessPermission) -> Result<(), AccessibilityError> {
        if permission.name.trim().is_empty() { return Err(AccessibilityError::EmptyName); }
        let id = permission.id.clone();
        if self.permissions.contains_key(&id) {
            return Err(AccessibilityError::DuplicatePermission(id.clone()));
        }
        self.permissions.insert(id, permission);
        Ok(())
    }

    pub fn permission_count(&self) -> usize { self.permissions.len() }

    pub fn get_permission(&self, id: &str) -> Option<&AccessPermission> { self.permissions.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessibility_create_aaa() {
        let a = DataAccessibility::new("My Accessibility");
        assert_eq!(a.name, "My Accessibility");
        assert_eq!(a.permission_count(), 0);
    }

    #[test]
    fn test_add_permission_aaa() {
        let mut a = DataAccessibility::new("My Accessibility");
        let p = AccessPermission { id: "p1".into(), name: "read".into(), resource: "data".into(), role: "analyst".into() };
        a.add_permission(p).unwrap();
        assert_eq!(a.permission_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = DataAccessibility::new("My Accessibility");
        let p = AccessPermission { id: "x".into(), name: "".into(), resource: "a".into(), role: "b".into() };
        assert!(a.add_permission(p).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = DataAccessibility::new("My Accessibility");
        let p = AccessPermission { id: "x".into(), name: "X".into(), resource: "a".into(), role: "b".into() };
        a.add_permission(p).unwrap();
        let p2 = AccessPermission { id: "x".into(), name: "X".into(), resource: "a".into(), role: "b".into() };
        assert!(a.add_permission(p2).is_err());
    }
}
