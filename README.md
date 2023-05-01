# 🩸 tinywad

A tiny lib to manage WAD file like DOOM

## 📖 How to build and run ?

1. Install the dependencies
    - `cargo`

## 💽 Compatibility

Official games status:

- `DOOM1.wad` 
- `DOOM2.wad` 
- `HEXEN.wad` 

Mods/unofficial games:

- `DBP37_AUGZEN.wad`

## ℹ️ Usage example

```rust
fn main() -> Result<(), WadError> {
    let mut wad = Wad::new();

    wad.set_palette(0);
    wad.load_from_file("wads/doom1.wad")?;
    wad.remove_by_name("^WILV*").unwrap();
    wad.save_lumps("./tmp");
    wad.save_lumps_raw("./tmp");
    wad.save("test.wad");

    Ok(())
}
```

## 🪧 Supported lump types

- DOOM image(s)
- Flat
- Palette
- Markers

## ✅ Todo

Name           | State
-------------  | :-------------:
Dump WAD header | ✅
Dump lumps metadata | ✅
Extract (save) lump | ✅
Update raw lump | ✅
Update lump | ❌
Rebuild then save the WAD as a new file | ✅
Extract sounds | ⚠️
Extract raw lump | ✅
Fix lumps linked hashmap | ✅
Add lump unique coherent IDs | ✅
Update lump size in the metadatas | ✅
Include static update into the WAD build process | ❌

## ℹ️ Documentation

Run `cargo doc --open` to read the documentation in the browser.
