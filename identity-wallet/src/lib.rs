pub mod command;
pub mod error;
pub mod http_client;
pub mod migrations;
pub mod persistence;
pub mod state;
pub mod stronghold;
pub mod subject;

// Re-exports
pub use oid4vc::{oid4vc_core, oid4vc_manager, oid4vci, oid4vp, siopv2};

use crate::error::AppError;

/// Helper function to open a URL in a browser tab.
/// On Android, it uses Chrome Custom Tabs (CCT) via `MainActivity.openCustomTab`.
/// On other platforms, it falls back to `tauri_plugin_opener`.
#[cfg(target_os = "android")]
pub fn open_url_in_browser<R: tauri::Runtime>(_app_handle: &tauri::AppHandle<R>, url: &str) -> Result<(), AppError> {
    use jni::objects::JValue;

    log::info!("open_url_in_browser (Android CCT): {url}");

    let android_context = ndk_context::android_context();
    let vm_ptr = android_context.vm();
    if vm_ptr.is_null() {
        log::error!("JavaVM pointer is null!");
        return Err(AppError::Error("JavaVM pointer is null".to_string()));
    }
    let vm = unsafe { jni::JavaVM::from_raw(vm_ptr.cast()) }
        .map_err(|e| AppError::Error(format!("Failed to get JavaVM: {e}")))?;

    let mut env = vm
        .attach_current_thread()
        .map_err(|e| AppError::Error(format!("Failed to attach thread: {e}")))?;

    let activity_ptr = android_context.context();
    if activity_ptr.is_null() {
        log::error!("Activity pointer is null!");
        return Err(AppError::Error("Activity pointer is null".to_string()));
    }
    let activity_obj = unsafe { jni::objects::JObject::from_raw(activity_ptr.cast()) };

    let url_jstring = env
        .new_string(url)
        .map_err(|e| AppError::Error(format!("Failed to create JString: {e}")))?;

    let res = env.call_method(
        &activity_obj,
        "openCustomTab",
        "(Ljava/lang/String;)V",
        &[JValue::Object(&url_jstring)],
    );

    if let Err(e) = res {
        log::error!("Failed to call openCustomTab: {e:?}");
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
        }
        return Err(AppError::Error(format!("Failed to call openCustomTab: {e}")));
    }

    log::info!("Successfully invoked openCustomTab");
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn open_url_in_browser<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>, url: &str) -> Result<(), AppError> {
    use tauri_plugin_opener::OpenerExt;
    app_handle
        .opener()
        .open_url(url, None::<&str>)
        .map_err(|err| AppError::Error(format!("Failed to open URL in browser: {err}")))
}

// This folder is where the main backend rust code lives together with all the business logic.
// The folder state is where our appstate and it's features are defined, completely according to the redux pattern.
// The command.rs holds the functions through which the front and backend comminicate using actions and reducers.
// The error.rs defines our app_error types, implemented throughout the code using the thiserror crate.
// The persistence.rs is where we define our app persistence functions.
// The stronghold.rs is where we implement the stronghold library for our app, which is used to store sensitive data.
