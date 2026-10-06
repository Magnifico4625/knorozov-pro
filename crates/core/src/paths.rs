//! Path helpers for C/C++ libraries that can't open non-ASCII paths on Windows.

use std::path::{Path, PathBuf};

/// Returns a path that native libraries using narrow (`char*`) file APIs can open.
///
/// On Windows, sherpa-onnx/onnxruntime open model files with the ANSI code page, so a path like
/// `C:\Users\Дамир\AppData\Local\Кнорозов PRO\models\x.onnx` fails. We try the 8.3 short path
/// first and, if that still isn't ASCII, copy the file to `%ProgramData%\KnorozovPRO`.
/// On other platforms the path is returned unchanged (UTF-8 works there).
pub fn native_safe_path(path: &Path) -> PathBuf {
    if path.to_str().map(|s| s.is_ascii()).unwrap_or(false) {
        return path.to_path_buf();
    }
    #[cfg(windows)]
    {
        if let Some(short) = short_path(path) {
            if short.to_str().map(|s| s.is_ascii()).unwrap_or(false) {
                return short;
            }
        }
        if let Some(pd) = std::env::var_os("ProgramData") {
            let dir = PathBuf::from(pd).join("KnorozovPRO");
            if let (Some(name), Ok(meta)) = (path.file_name(), std::fs::metadata(path)) {
                let dst = dir.join(name);
                let same = std::fs::metadata(&dst).map(|m| m.len() == meta.len()).unwrap_or(false);
                if same || (std::fs::create_dir_all(&dir).is_ok() && std::fs::copy(path, &dst).is_ok()) {
                    if dst.to_str().map(|s| s.is_ascii()).unwrap_or(false) {
                        return dst;
                    }
                }
            }
        }
    }
    path.to_path_buf()
}

#[cfg(windows)]
fn short_path(path: &Path) -> Option<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use windows_sys::Win32::Storage::FileSystem::GetShortPathNameW;
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let mut buf = vec![0u16; 1024];
    let n = unsafe { GetShortPathNameW(wide.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() {
        return None;
    }
    buf.truncate(n);
    Some(PathBuf::from(OsString::from_wide(&buf)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ascii_unchanged() {
        let p = Path::new("/tmp/model.onnx");
        assert_eq!(native_safe_path(p), p);
    }
}
