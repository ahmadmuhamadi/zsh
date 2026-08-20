use std::ffi::CStr;
use std::fs;
use std::os::raw::{c_char, c_int};
use std::path::Path;

use pdf_inspector::process_pdf;

fn pdf_to_markdown_file(input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = process_pdf(input_path)?;

    let markdown = result
        .markdown
        .ok_or("No markdown produced (scanned/non-text PDF?)")?;

    let out_path = Path::new(input_path).with_extension("md");
    fs::write(&out_path, markdown)?;

    Ok(())
}

/// Reads the PDF at `path` and writes a sibling `.md` file next to it.
/// Returns 0 on success, -1 if `path` is null or not valid UTF-8, -2 if
/// conversion failed (error details are printed to stderr).
#[unsafe(no_mangle)]
pub extern "C" fn rust_convert_pdf_to_markdown(path: *const c_char) -> c_int {
    if path.is_null() {
        return -1;
    }

    let path = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(p) => p,
        Err(_) => return -1,
    };

    match pdf_to_markdown_file(path) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("heredoc_helper: failed to convert {path}: {e}");
            -2
        }
    }
}
