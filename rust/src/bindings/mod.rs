#[cfg(target_os = "android")]
mod android;

// we call the ios functions from the android ones
mod ios;
mod ts_fns;
