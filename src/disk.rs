//! Quick file system operations, with pretty logging.

use crate::serror;
use crate as fox;
use std::path::{Path, PathBuf};

/// Deletes the given file.
pub fn delete_file<P: AsRef<Path>>(file_path: P) -> Result<(), std::io::Error> {
    let path = file_path.as_ref();
    std::fs::remove_file(path).map_err(|err| {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                serror!("File `{}` not found for deletion.", path.display());
            }
            std::io::ErrorKind::PermissionDenied => {
                serror!("Not permitted to delete file `{}`.", path.display());
            }
            std::io::ErrorKind::IsADirectory => {
                serror!("Cannot delete `{}`, as it is a directory.", path.display());
            }
            _ => {
                serror!("Failed to delete file `{}`: {}", path.display(), err);
            }
        }

        err
    })
}

/// Reads the metadata of a file.
pub fn file_info<P: AsRef<Path>>(file_path: P) -> Result<std::fs::Metadata, std::io::Error> {
    let path = file_path.as_ref();
    std::fs::metadata(path).map_err(|err| {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                serror!("File `{}` not found.", path.display());
            }
            std::io::ErrorKind::PermissionDenied => {
                serror!("Not permitted to read metadata of file `{}`.", path.display());
            }
            _ => {
                serror!("Failed to read metadata of file `{}`: {}", path.display(), err);
            }
        }

        err
    })
}

/// Lists the content of a directory.
pub fn list_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let path = path.as_ref();
    let entries = std::fs::read_dir(path).map_err(|err| {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                serror!("Directory `{}` not found.", path.display());
            }
            std::io::ErrorKind::PermissionDenied => {
                serror!("Not permitted to read directory `{}`.", path.display());
            }
            _ => {
                serror!("Failed to read directory `{}`: {}", path.display(), err);
            }
        }

        err
    })?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        files.push(entry_path);
    }

    files.sort();
    Ok(files)
}

/// Lists the content of a directory, recursively.
pub fn list_dir_all<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let root_path = path.as_ref();
    let mut files = Vec::new();

    fn read_dir_recursive(path: &Path, result: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
        let entries = std::fs::read_dir(path).map_err(|err| {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    serror!("Directory `{}` not found.", path.display());
                }
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to read directory `{}`.", path.display());
                }
                _ => {
                    serror!("Failed to read directory `{}`: {}", path.display(), err);
                }
            }

            err
        })?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            result.push(entry_path.clone());

            if entry_path.is_dir() {
                read_dir_recursive(&entry_path, result)?;
            }
        }
        Ok(())
    }

    read_dir_recursive(root_path, &mut files)?;
    files.sort();
    Ok(files)
}

/// Lists the content of a directory, recursively. Returns relative paths from the root.
pub fn list_dir_all_relative<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let root_path = path.as_ref();
    let all_paths = list_dir_all(root_path)?;

    let relative_paths = all_paths
        .into_iter()
        .filter_map(|p| p.strip_prefix(root_path).ok().map(|p| p.to_path_buf()))
        .collect();

    Ok(relative_paths)
}
