// Copyright 2017 ttf-noto-sans Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Noto Sans TTF fonts for embedding into your application binary.

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
pub const BOLD: &'static [u8] = include_bytes!("../assets/NotoSans-Bold.rrf");

/// Noto Sans Italic
#[cfg(feature = "italic")]
pub const ITALIC: &'static [u8] = include_bytes!("../assets/NotoSans-Italic.rrf");

/// Noto Sans Bold Italic
#[cfg(feature = "bold_italic")]
pub const BOLD_ITALIC: &'static [u8] = include_bytes!("../assets/NotoSans-BoldItalic.rrf");
