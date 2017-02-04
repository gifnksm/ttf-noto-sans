# Noto Sans TrueType fonts for Rust applications

[![Travis CI Build Status](https://travis-ci.org/gifnksm/ttf-noto-sans.svg?branch=master)](https://travis-ci.org/gifnksm/ttf-noto-sans)
[![crates.io](https://img.shields.io/crates/v/ttf-noto-sans.svg)](https://crates.io/crates/ttf-noto-sans)

You can use embedded font data from your application.

By default, regular style font is embedded.

```toml
[package]
ttf-noto-sans = "0.1"
```

You can also use other style fonts by specifying custom features.

```toml
[package]
ttf-noto-sans = { version = "0.1", features = ["bold", "italic", "bold_italic"] }
```

## Example (use with [conrod](https://github.com/PistonDevelopers/conrod))

```rust
extern crate conrod;
use conrod::UiBuilder;
use conrod::text::FontCollection;

let mut ui = UiBuilder::new([640.0, 480.0]).build();
let _ = ui.fonts.insert(FontCollection::from_bytes(ttf_noto_sans::REGULAR)
    .into_font()
    .expect("failed to into_font"));
```

Complete example is [here](examples/conrod_text.rs)
