//! Android entry point for FilmCraft.
//!
//! Android's `NativeActivity` loads this library and calls [`android_main`] on a dedicated
//! thread. It opens the shared app ([`filmcraft::run_android`], the same code the desktop binary
//! runs) and returns when the window closes, ending the activity's main-loop thread.
//!
//! This crate exists because that export must be unmangled, and on edition 2024 the only spelling
//! (`#[unsafe(no_mangle)]`) trips the workspace-forbidden `unsafe_code` lint — see `Cargo.toml`
//! for why the crate carries its own lints instead. It holds no unsafe code of its own.
//!
//! Android limitations (v1): no system file picker and no system message dialogs (rfd has no
//! Android backend), so Save, Open Project, exports, folder picks and media import are served by
//! FilmCraft's own in-app browser over app-private storage; links and "open path" report why they
//! cannot open (no `ACTION_VIEW` intent without JNI); and there is no per-user data folder, so
//! logs, auto-save and settings live in the app-private storage directory the activity resolves.
//!
//! On every other target this crate is empty: only Android's loader ever looks for `android_main`.

/// NativeActivity's main-loop thread entry point (required; see the `android-activity` docs).
/// winit pumps the activity's lifecycle and input events inside [`filmcraft::run_android`], which
/// needs the activity's handle to create its event loop; when the activity is destroyed that
/// returns and this returns with it.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: android_activity::AndroidApp) {
    if let Err(e) = filmcraft::run_android(app) {
        log::error!("filmcraft exited with an error: {e}");
    }
}
