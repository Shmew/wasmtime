//! ARM64 Settings.

use crate::settings::{self, detail, Builder};
use std::fmt;

// Include code generated by `lib/codegen/meta-python/gen_settings.py`. This file contains a public
// `Flags` struct with an impl for all of the settings defined in
// `lib/codegen/meta-python/isa/arm64/settings.py`.
include!(concat!(env!("OUT_DIR"), "/settings-arm64.rs"));
