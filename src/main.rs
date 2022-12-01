use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use regex::Regex;

use tinywad::error::WadError;
use tinywad::models::operation::WadOperation;
use tinywad::wad::{
    Wad,
    DEFAULT_RE_NAME, WadOperationKind
};

#[derive(StructOpt, Debug)]
#[structopt(name = "tinywad")]
struct Opt {
    /// Input WAD file
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Match lumps name regex, matching everything by default
    #[structopt(long)]
    re: Option<String>,
    /// Operation <dump, save, save_as>, dump by default
    #[structopt(long)]
    op: Option<WadOperationKind>,
    /// Optional output directory
    #[structopt(long)]
    dir: Option<String>,
    /// Custom palette index, 0 by default
    #[structopt(long)]
    pal: Option<usize>,
}

impl Opt {
    /// Get the regex string value
    pub fn re(&self) -> Result<Regex, WadError> {
        // Unwraping the optional CLI argument
        let re_result = if self.re.is_none() {
            Regex::new(DEFAULT_RE_NAME)
        } else {
            let arg_value = &self.re.clone().unwrap();

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
        self.op.unwrap_or_default()
    }

    /// Get the palette index
    pub fn pal(&self) -> usize {
        self.pal.unwrap_or(0)
    }
}


fn main() -> Result<(), WadError> {
    // Arguments
    let args = Opt::from_args();
    let re = args.re()?;
    let op = args.op();

    // WAD manager
    let mut wad = Wad::new(re);

    wad.set_palette(args.pal());
    wad.load_from_file(args.path)?;

    match op {
        WadOperationKind::Dump => wad.dump(),
        WadOperationKind::Save => wad.save(),
        WadOperationKind::SaveAs => {
            if args.dir.is_none() {
                wad.save()
            } else {

                let dirs = args.dir.unwrap();
                let created_dirs = fs::create_dir_all(dirs.clone());

                if created_dirs.is_err() {
                    return Err(WadError::Write);
                } else {
                    created_dirs.unwrap();
                }

                wad.save_as(dirs);
            }
        }
    };

    Ok(())
}
