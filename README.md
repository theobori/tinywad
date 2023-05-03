# 🩸 tinywad

A tiny lib to manage WAD file like DOOM1/2, HEXEN, etc..

It supports the following features:
- Load WAD buffer/file
- Extract lump raw content
- Extract lump as original files (PNGs)
- Extract the image lumps with a custom color palette
- Update lump raw content
- Build a IWAD/PWAD
- Add/insert lumps then save the WAD file

## 📖 How to build and run ?

1. Install the dependencies
    - `cargo`


## ⭐ Use cases

#### Patching directly the IWAD

```rust
use tinywad::error::WadError;
use tinywad::models::operation::WadOp;
use tinywad::wad::Wad;

fn main() -> Result<(), WadError> {
    let mut doom_2 = Wad::new();
    doom_2.load_from_file("wads/doom2.wad")?;

    let gate = doom_2.lump("GATE3").unwrap();

    let mut doom_1 = Wad::new();
    doom_1.load_from_file("doom1.wad")?;

    doom_1.select("^FLAT|FLOOR");
    doom_1.update_lumps_raw(&gate.data().buffer);
    doom_1.save("doom1.wad");

    Ok(())
}
```

#### Extracting lumps with custom palettes

```rust
use std::fs;

use tinywad::dir::MAX_PAL;
use tinywad::error::WadError;
use tinywad::models::operation::WadOp;
use tinywad::wad::Wad;

fn main() -> Result<(), WadError> {
    let mut doom_2 = Wad::new();

    for pal in 0..MAX_PAL {
        doom_2.set_palette(pal);
        doom_2.load_from_file("wads/doom2.wad")?;
        doom_2.select("^BOSF");
        
        let dirpath = format!("doom2/pal_{}", pal);

        fs::create_dir_all(dirpath.clone()).unwrap();

        doom_2.save_lumps(dirpath);
        //doom_2.save_lumps_raw(dirpath);
    }

    Ok(())
}
```

#### Building a PWAD from scratch

```rust
use tinywad::error::WadError;
use tinywad::lump::{LumpAdd, LumpAddKind};
use tinywad::models::operation::WadOp;
use tinywad::wad::{Wad, WadKind,};

fn main() -> Result<(), WadError> {
    let mut src = Wad::new();

    let lump_names = [
        "FLOOR0_1", "FLOOR0_3", "FLOOR0_6",
        "FLOOR1_1", "FLOOR1_7", "FLOOR3_3",
        "FLOOR4_1", "FLOOR4_5", "FLOOR4_6",
        "FLOOR4_8", "FLOOR5_1", "FLOOR5_2",
        "FLOOR5_3", "FLOOR5_4", "FLOOR6_1",
        "FLOOR6_2", "FLOOR7_1", "FLOOR7_2",
    ];

    src.load_from_file("doom.wad")?;

    let gate = src.lump("FLOOR6_1").unwrap();

    let mut dest = Wad::new();

    dest.set_kind(WadKind::Pwad);
    dest.add_lump_raw(
        LumpAdd::new(
            LumpAddKind::Back,
            &vec![],
            "FF_START",
        )
    )?;

    for lump_name in lump_names {
        dest.add_lump_raw(
            LumpAdd::new(
                LumpAddKind::Back,
                &gate.data().buffer,
                lump_name,
            )
        )?;
    }

    dest.add_lump_raw(
        LumpAdd::new(
            LumpAddKind::Back,
            &vec![],
            "F_END",
        )
    )?;

    dest.save("doom1_patch.wad");

    Ok(())
}
```

#### Dumping metadata

```rust
use tinywad::error::WadError;
use tinywad::models::operation::WadOp;
use tinywad::wad::Wad;

fn main() -> Result<(), WadError> {
    let mut src = Wad::new();

    src.load_from_file("hexen.wad")?;
    src.dump();

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
Update lump from raw buffer/file| ✅
Update lump from original buffer/files (PNGs) | ❌
Rebuild then save the WAD as a new file | ✅
Extract sounds | ⚠️
Extract raw lump | ✅
Remove lumps | ✅
Add lump unique coherent IDs | ✅
Update lump size in the metadatas | ✅
Add lump with raw buffer | ✅

## ℹ️ Documentation

Run `cargo doc --open` to read the documentation in the browser.
