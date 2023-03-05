// Copyright 2017 ttf-noto-sans Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Noto Sans True Type fonts for embedding into your application binary.
//!
//! By default, regular style font is embedded.
//!
//! ```toml
//! [package]
//! ttf-noto-sans = "0.1"
//! ```
//!
//! You can also use other style fonts by specifying custom features.
//!
//! ```toml
//! [package]
//! ttf-noto-sans = { version = "0.1", features = ["bold", "italic", "bold_italic"] }
//! ```
//!
//! ## Example (use with [conrod](https://github.com/PistonDevelopers/conrod))
//!
//! ```rust
//! # extern crate conrod;
//! # extern crate ttf_noto_sans;
//! # fn main() {
//! use conrod::UiBuilder;
//! use conrod::text::FontCollection;
//!
//! let mut ui = UiBuilder::new([640.0, 480.0]).build();
//! let _ = ui.fonts.insert(FontCollection::from_bytes(ttf_noto_sans::REGULAR)
//!     .into_font()
//!     .expect("failed to into_font"));
//! # }
//! ```
//!
//! Complete example is [here](examples/conrod_text.rs)

#![warn(bad_style)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

/// Noto Sans Regular
#[cfg(feature = "regular")]
pub const REGULAR: &'static [u8] = include_bytes!("../assets/NotoSans-Regular.ttf");

/// Noto Sans Bold
#[cfg(feature = "bold")]
pub const BOLD: &'static [u8] = include_bytes!("../assets/NotoSans-Bold.ttf");

/// Noto Sans Italic
#[cfg(feature = "italic")]
pub const ITALIC: &'static [u8] = include_bytes!("../assets/NotoSans-Italic.ttf");

/// Noto Sans Bold Italic
#[cfg(feature = "bold_italic")]
pub const BOLD_ITALIC: &'static [u8] = include_bytes!("../assets/NotoSans-BoldItalic.ttf");
