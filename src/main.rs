use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use regex::Regex;

use tinywad::error::WadError;
use tinywad::models::operation::WadOperation;
use tinywad::wad::{
    Wad,
    DEFAULT_RE_NAME,
    WadOperationKind
};

#[derive(StructOpt, Debug)]
#[structopt(name = "tinywad")]
struct Opt {
    /// Input WAD file
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Match lumps name regex, matching everything by default
    #[structopt(long)]
    regex: Option<String>,
    /// Operation <dump, save, save_as>, dump by default
    #[structopt(long)]
    operation: Option<WadOperationKind>,
    /// Optional output directory
    #[structopt(long)]
    dir: Option<String>,
    /// Custom palette index, 0 by default
    #[structopt(long)]
    palette: Option<usize>,
}

impl Opt {
    /// Get the regex string value
    pub fn regex_obj(&self) -> Result<Regex, WadError> {
        // Unwraping the optional CLI argument
        let re_result = if self.regex.is_none() {
            Regex::new(DEFAULT_RE_NAME)
        } else {
            let arg_value = &self.regex
                .clone()
                .unwrap();

            Regex::from_str(arg_value)
        };

        // Unwrapping the Regex value
        if re_result.is_err() {
            Err(WadError::InvalidRegex)
        } else {
            Ok(re_result.unwrap())
        }
    }

    /// Get the operation
    pub fn op(&self) -> WadOperationKind {
        self.operation.unwrap_or_default()
    }

    /// Get the palette index
    pub fn pal(&self) -> usize {
        self.palette.unwrap_or(0)
    }
}

/// Create directories if needed
fn create_dirs(dirname: String) -> Result<(), WadError> {
    let created_dirs = fs::create_dir_all(dirname);

    if created_dirs.is_err() {
        return Err(WadError::Write);
    } else {
        created_dirs.unwrap();
    }

    Ok(())
}

// fn main() -> Result<(), WadError> {
//     // Arguments
//     let args = Opt::from_args();
//     let op = args.op();

//     // WAD manager
//     let mut wad = Wad::new();

//     wad.set_re_name("^FLOOR*");
//     wad.set_palette(args.pal());
//     wad.load_from_file(args.path)?;

//     match op {
//         WadOperationKind::Dump => wad.dump(),
//         WadOperationKind::Save => wad.save(),
//         WadOperationKind::SaveAs => {
//             match args.dir {
//                 Some(dirname) => {
//                     create_dirs(dirname.clone())?;
//                     wad.save_as(dirname);
//                 },
//                 None => wad.save()
//             }
//         }
//     };

//     Ok(())
// }

fn main() -> Result<(), WadError> {
    let mut wad = Wad::new();

    // wad.set_re_name("^FLOOR*");
    wad.set_palette(0);
    wad.load_from_file("wads/doom1.wad")?;

    wad.save("./tmp");

    // let contents = wad.buffer.borrow().clone();

    // fs::write(
    //     "test.raw",
    //     contents
    // );

    Ok(())
}