# 🩸 tinywad

![build](https://github.com/theobori/tinywad/actions/workflows/build.yml/badge.svg)


A library to manage WAD for DOOM based game. It has been tested with the following IWAD:
- DOOM.WAD
- DOOM2.WAD
- HEXEN.WAD
- CHEX.WAD
- TNT.WAD
- PLUTONIA.WAD
- STRIFE0.WAD
- STRIFE1.WAD
- DOOMU.WAD
- DOOM2F.WAD
- HEXDD.WAD

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
    doom_2.load_from_file("doom2.wad")?;

    let gate = doom_2.lump("GATE3").unwrap();

    let mut doom_1 = Wad::new();
    doom_1.load_from_file("doom1.wad")?;

    doom_1.select("^FLAT|FLOOR");
    doom_1.update_lumps_raw(&gate.data().buffer);
    doom_1.save("doom1.wad");

    Ok(())
}
```

#### Screenshot(s)

<img src="/assets/doom_gate3.png" width="60%">

#### Extracting lumps with custom palettes

```rust
use std::fs;

use tinywad::dir::MAX_PAL;
use tinywad::error::WadError;
use tinywad::models::operation::WadOp;
use tinywad::wad::Wad;

fn main() -> Result<(), WadError> {
    let mut doom_2 = Wad::new();
    doom_2.load_from_file("doom2.wad")?;

    for pal in 0..MAX_PAL {
        doom_2.set_palette(pal);
        doom_2.reload()?;
        doom_2.select("^BOSF");
        
        let dirpath = format!("doom2/pal_{}", pal);

        fs::create_dir_all(dirpath.clone()).unwrap();

        doom_2.save_lumps(dirpath);
    }

    Ok(())
}
```

#### Extracted lumps (as PNGs)

<p float="left">
    <img src="/assets/doom2/pal_0/BOSFB0.png">
    <img src="/assets/doom2/pal_1/BOSFB0.png">
    <img src="/assets/doom2/pal_2/BOSFB0.png">
    <img src="/assets/doom2/pal_3/BOSFB0.png">
    <img src="/assets/doom2/pal_4/BOSFB0.png">
    <img src="/assets/doom2/pal_5/BOSFB0.png">
    <img src="/assets/doom2/pal_6/BOSFB0.png">
    <img src="/assets/doom2/pal_7/BOSFB0.png">
    <img src="/assets/doom2/pal_8/BOSFB0.png">
    <img src="/assets/doom2/pal_9/BOSFB0.png">
    <img src="/assets/doom2/pal_10/BOSFB0.png">
    <img src="/assets/doom2/pal_11/BOSFB0.png">
    <img src="/assets/doom2/pal_12/BOSFB0.png">
</p>

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

<img src="/assets/doom_floor6_1.png" width="60%">

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

#### Output

```
Name: XXTIC, Size: 8, Offset: 12
Name: STARTUP, Size: 153648, Offset: 20
Name: PLAYPAL, Size: 21504, Offset: 153668, Palettes amount: 28
Name: COLORMAP, Size: 8704, Offset: 175172
Name: FOGMAP, Size: 8704, Offset: 183876
Name: TINTTAB, Size: 65536, Offset: 192580
Name: TRANTBL0, Size: 256, Offset: 258116
Name: TRANTBL1, Size: 256, Offset: 258372
Name: TRANTBL2, Size: 256, Offset: 258628
...
```

## 🪧 Supported lump types

- DOOM image(s)
- Flat
- Palette
- Markers
- Music

## ✅ Todo

Name           | State
-------------  | :-------------:
Dump WAD header | ✅
Dump lumps metadata | ✅
Extract (save) lump | ✅
Update lump from raw buffer/file| ✅
Update lump from original buffer/files (PNG, MIDI, etc..) | ❌
Rebuild then save the WAD as a new file | ✅
Extract DOOM musics | ✅
Extract raw lump | ✅
Remove lumps | ✅
Add lump unique coherent IDs | ✅
Update lump size in the metadatas | ✅
Add lump with raw buffer | ✅

## ℹ️ Documentation

Run `cargo doc --open` to read the documentation in the browser.
