# 🩸 tinywad

A tiny tool to make some WAD extraction like images, sounds, etc..

## 📖 How to build and run ?

1. Install the dependencies
    - `cargo`

## 📖 Usage example

```rust
fn main() -> Result<(), WadError> {
    let mut wad = Wad::new();

    // Setup the manager
    wad.set_re_name("^FLOOR*");
    wad.set_palette(0);

    // Load a wad file
    wad.load_from_file("wads/doom1.wad")?;
    
    // Then save every `FLOOR` lumps as files
    wad.save(".");

    Ok(())
}
```

## 🪧 Supported lump types

- DOOM image(s)
- Flat
- Palette
- Markers

## ✅ Todo

Name           | Status
-------------  | :-------------:
Dump WAD header | ✅
Dump lumps metadata | ✅
Extract (save) lump | ✅
Update lump | ⚠️
Extract sounds | ⚠️
Extract raw lump | ✅

## ℹ️ Documentation

Run `cargo doc --open` to read the documentation in the browser.
