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
TODO

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
