//! A build distributed by someone other than the project: a derivative with its own GitHub
//! releases, version marks and (on macOS) its own Developer ID. Set at build time; a build
//! without them updates from the project's releases, as before.
//!
//! - `SOLIDCRAFT_UPDATE_REPO` (`owner/name`): check and download only this repository's releases.
//! - `SOLIDCRAFT_UPDATE_TAG_MARK` (`xpt`): a tag such as `v0.23.0-xpt.1` is a release, not a
//!   prerelease; `xpt.2` follows `xpt.1`.
//! - `SOLIDCRAFT_UPDATE_MACOS_DMG` (`SolidCraft-{version}-macos-aarch64.dmg`): the disk image's
//!   name in a release, `{version}` being the tag without its `v`.
//! - `SOLIDCRAFT_UPDATE_MACOS_TEAM` (an Apple Team ID): the app is Developer ID signed and
//!   notarized by this team, so Update now replaces the bundle in place, but only with an app
//!   signed by the same team under the same bundle identifier (install.rs).

pub const REPO: Option<&str> = option_env!("SOLIDCRAFT_UPDATE_REPO");
pub const TAG_MARK: Option<&str> = option_env!("SOLIDCRAFT_UPDATE_TAG_MARK");
pub const MACOS_DMG: Option<&str> = option_env!("SOLIDCRAFT_UPDATE_MACOS_DMG");
pub const MACOS_TEAM: Option<&str> = option_env!("SOLIDCRAFT_UPDATE_MACOS_TEAM");

/// `pre` (a version's prerelease part) is `mark` or `mark.<anything>`: a release of the
/// derivative, not a prerelease.
pub fn is_marked_release(pre: &semver::Prerelease, mark: Option<&str>) -> bool {
    let Some(mark) = mark.filter(|m| !m.is_empty()) else { return false };
    let pre = pre.as_str();
    pre == mark || pre.strip_prefix(mark).is_some_and(|rest| rest.starts_with('.'))
}

/// The macOS disk image of `version` under `template`, else the project's name for it.
pub fn macos_dmg(template: Option<&str>, version: &str) -> String {
    template.filter(|t| !t.is_empty()).map_or_else(|| "solidcraft-macos-universal.dmg".to_string(), |t| t.replace("{version}", version))
}

/// Release pages and assets of `repo` (`owner/name`): `https://github.com/owner/name/releases/`.
pub fn release_prefix(repo: &str) -> String {
    format!("https://github.com/{repo}/releases/")
}
