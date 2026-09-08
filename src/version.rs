//! Версия приложения в формате `vX.Y.Z-<hash>` (см. RULES.md §6).

/// Полная строка версии, например `v0.1.0-f7eacc9`.
pub const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"));

#[inline]
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_format_is_v_semver_hash() {
        assert!(
            VERSION.starts_with('v'),
            "версия начинается с 'v': {VERSION}"
        );
        assert!(VERSION.contains('-'), "версия содержит хеш: {VERSION}");
        assert!(!VERSION.ends_with('-'), "хеш не пустой: {VERSION}");
    }
}
