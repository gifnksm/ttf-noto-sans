# Noto Sans TTF fonts for embedding into your application binary.

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
use conrod::UiBuilder;
use conrod::text::FontCollection;

let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
let _ = ui.fonts.insert(FontCollection::from_bytes(ttf_noto_sans::REGULAR)
    .into_font()
    .expect("failed to into_font"));
```

Complete example is [here](examples/conrod_text.rs)
