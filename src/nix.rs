use libc::c_char;

use errno::Errno;

#[derive(Copy)]
pub enum NixError {
    Sys(Errno),
    InvalidPath
}

pub trait NixPath {
    fn with_nix_path<T, F>(&self, f: F) -> Result<T, NixError>
        where F: FnOnce(*const c_char) -> T;
}
