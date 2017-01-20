extern crate zip;
extern crate libc;

use std::io;
use std::fs;
use std::path::{Path, PathBuf, Component};
use std::ffi::CStr;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[no_mangle]
pub extern fn extract(zippath: *const libc::c_char, outdirpath: *const libc::c_char) {
    let zname = Path::new(rust_string(zippath));
    let outdir = Path::new(rust_string(outdirpath));

    create_directory(&outdir, None);
    let file = fs::File::open(&zname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len()
    {
        let mut file = archive.by_index(i).unwrap();
        let outpath = sanitize_filename(file.name(), outdir);

        create_directory(outpath.parent().unwrap_or(Path::new("")), None);

        let perms = convert_permissions(file.unix_mode());

        if (&*file.name()).ends_with("/") {
            create_directory(&outpath, perms);
        }
        else {
            write_file(&mut file, &outpath, perms);
        }
    }
}

fn rust_string(r_string: *const libc::c_char) -> &'static str {
  unsafe { CStr::from_ptr(r_string) }.to_str().unwrap()
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

fn write_file(file: &mut zip::read::ZipFile, outpath: &Path, perms: Option<fs::Permissions>)
{
    let mut outfile = fs::File::create(&outpath).unwrap();
    io::copy(file, &mut outfile).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
}

fn create_directory(outpath: &Path, perms: Option<fs::Permissions>)
{
    fs::create_dir_all(&outpath).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
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
