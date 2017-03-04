extern crate zip;

use std::io;
use std::fs;
use std::path::{Path, PathBuf, Component};
use std::error::Error;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn run(zippath: &Path, outdirpath: &Path) -> Result<bool, Box<Error>>
{
    let file = try!(fs::File::open(&zippath));
    try!(create_directory(&outdirpath, None));

    let mut archive = try!(zip::ZipArchive::new(file));

    for i in 0..archive.len()
    {
        let mut file = archive.by_index(i).unwrap();
        let outpath = sanitize_filename(file.name(), outdirpath);

        try!(create_directory(outpath.parent().unwrap_or(Path::new("")), None));

        let perms = convert_permissions(file.unix_mode());

        if file.name().ends_with("/") {
            try!(create_directory(&outpath, perms));
        } else {
            try!(write_file(&mut file, &outpath, perms));
        }
    }

    Ok(true)
}

#[cfg(unix)]
fn convert_permissions(mode: Option<u32>) -> Option<fs::Permissions>
{
    match mode {
        Some(mode) => Some(fs::Permissions::from_mode(mode)),
        None => None,
    }
}

#[cfg(not(unix))]
fn convert_permissions(_mode: Option<u32>) -> Option<fs::Permissions>
{
    None
}

fn write_file(file: &mut zip::read::ZipFile, outpath: &Path, perms: Option<fs::Permissions>) -> Result<(), Box<Error>>
{
    let mut outfile = try!(fs::File::create(&outpath));
    try!(io::copy(file, &mut outfile));

    if let Some(perms) = perms {
        try!(fs::set_permissions(outpath, perms));
    }

    Ok(())
}

fn create_directory(outpath: &Path, perms: Option<fs::Permissions>) -> Result<(), Box<Error>>
{
    try!(fs::create_dir_all(&outpath));

    if let Some(perms) = perms {
        try!(fs::set_permissions(outpath, perms));
    }

    Ok(())
}

fn sanitize_filename(filename: &str, outdir: &Path) -> PathBuf
{
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    let filepath = Path::new(no_null_filename)
        .components()
        .filter(|component| *component != Component::ParentDir)
        .fold(PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        });

    let mut outdirbuf = PathBuf::from(outdir);
    outdirbuf.push(filepath);
    return outdirbuf.to_path_buf();
}
