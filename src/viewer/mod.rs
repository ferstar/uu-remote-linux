//! Opt-in controller session; independent of the host room and peer lifecycle.
pub mod api;
pub mod assist;
pub mod rtc;
pub mod session;
pub mod signal;
#[cfg(test)]
mod tests;
pub mod wire;

pub mod audio;
#[cfg(feature = "viewer-ffmpeg")]
pub mod decoder;
#[cfg(any(test, feature = "viewer-sdl"))]
pub mod decoder_cli;
pub mod desktop;
#[cfg(uu_gtk)]
pub mod host;
#[cfg(feature = "viewer-sdl")]
pub mod hud;
pub mod input;
pub mod media;
#[cfg(feature = "viewer-sdl")]
pub mod sdl;
pub mod thumbs;
#[cfg(uu_gtk)]
pub mod tools_ui;
#[cfg(uu_gtk)]
pub mod ui;

pub mod native_gamepad;
pub mod native_rumble;
pub mod tools;

pub mod clipboard;
pub mod cursor;
pub mod power;
pub mod secure_input;
pub mod type_ipc;

fn device_label(name: Option<&str>) -> &str {
    name.filter(|name| !name.is_empty())
        .unwrap_or_else(|| crate::i18n::s("远端设备", "Remote device"))
}

#[cfg(test)]
mod device_label_tests {
    #[test]
    fn device_name_or_fallback() {
        assert_eq!(super::device_label(Some("测试电脑")), "测试电脑");
        assert_eq!(
            super::device_label(None),
            crate::i18n::s("远端设备", "Remote device")
        );
        assert_eq!(
            super::device_label(Some("")),
            crate::i18n::s("远端设备", "Remote device")
        );
    }
}
