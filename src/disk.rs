//! File system operations, without the hassle.

use crate::{critical, warn, pretext};
use crate as fox;

/// Returns the content of a file as a string, or crashes with a `critical!` on failure.
/// Useful for cases where a file must exist and be readable, or the program should not continue.
///
/// Requires valid UTF-8 content.
pub fn read_file(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            content
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    critical!("File `{}` not found.", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to read file `{}`.", file_path);
                }
                std::io::ErrorKind::InvalidData => {
                    critical!("File `{}` is not valid UTF-8.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    critical!("Cannot read `{}`, as it is a directory.", file_path);
                }
                _ => {
                    critical!("Failed to read file `{}`: {}", file_path, e);
                }
            }
            std::process::exit(1);
        }
    }
}

/// Writes the given content to a file, or crashes with a `critical!` on failure.
/// Useful for cases where a file must be written to, or the program should not continue.
///
/// Overwrites the file if it already exists.
pub fn write_file(file_path: &str, content: &str) {
    match std::fs::write(file_path, content) {
        Ok(_) => {}
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to write to file `{}`.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    critical!("Cannot write to `{}`, as it is a directory.", file_path);
                }
                _ => {
                    critical!("Failed to write to file `{}`: {}", file_path, e);
                }
            }
            std::process::exit(1);
        }
    }
}

/// Deletes the given file, or crashes with a `critical!` on failure.
/// Does not crash if the file does not exist, but does print a `warn!`.
///
/// Useful for cases where a file should be deleted, but it's not a problem if it doesn't exist.
pub fn delete_file(file_path: &str) {
    match std::fs::remove_file(file_path) {
        Ok(_) => {}
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    warn!("File `{}` not found for deletion.", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to delete file `{}`.", file_path);
                    std::process::exit(1);
                }
                std::io::ErrorKind::IsADirectory => {
                    critical!("Cannot delete `{}`, as it is a directory.", file_path);
                    std::process::exit(1);
                }
                _ => {
                    critical!("Failed to delete file `{}`: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Reads the metadata of a file, or crashes with a `critical!` on failure.
///
/// Returns `std::fs::Metadata`.
pub fn file_info(file_path: &str) -> Option<std::fs::Metadata> {
    match std::fs::metadata(file_path) {
        Ok(metadata) => {
            Some(metadata)
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    critical!("File `{}` not found.", file_path);
                    std::process::exit(1);
                }
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to read metadata of file `{}`.", file_path);
                    std::process::exit(1);
                }
                _ => {
                    critical!("Failed to read metadata of file `{}`: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Reads the metadata of a file, or returns `None` if the file does not exist.
///
/// Returns `std::fs::Metadata` or `None`.
pub fn file_info_or_none(file_path: &str) -> Option<std::fs::Metadata> {
    match std::fs::metadata(file_path) {
        Ok(metadata) => {
            Some(metadata)
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    None
                }
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to read metadata of file `{}`.", file_path);
                    std::process::exit(1);
                }
                _ => {
                    critical!("Failed to read metadata of file `{}`: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Lists the content of a directory, or crashes with a `critical!` on failure.
pub fn list_dir(path: &str) -> Vec<String> {
    match std::fs::read_dir(path) {
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    match entry {
                        Ok(entry) => {
                            match entry.file_name().into_string() {
                                Ok(file_name) => {
                                    Some(file_name)
                                }
                                Err(_) => {
                                    warn!("Failed to read file name in directory `{}`.", path);
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to read directory `{}`: {}", path, e);
                            None
                        }
                    }
                })
                .collect()
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    critical!("Directory `{}` not found.", path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to read directory `{}`.", path);
                }
                _ => {
                    critical!("Failed to read directory `{}`: {}", path, e);
                }
            }
            std::process::exit(1);
        }
    }
}

/// Lists the content of a directory, or returns an empty vector if it doesn't exist. Crashes with a `critical!` on failure.
pub fn list_dir_or_empty(path: &str) -> Vec<String> {
    match std::fs::read_dir(path) {
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    match entry {
                        Ok(entry) => {
                            match entry.file_name().into_string() {
                                Ok(file_name) => {
                                    Some(file_name)
                                }
                                Err(_) => {
                                    warn!("Failed to read file name in directory `{}`.", path);
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to read directory `{}`: {}", path, e);
                            None
                        }
                    }
                })
                .collect()
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    warn!("Directory `{}` not found.", path);
                    vec![]
                }
                std::io::ErrorKind::PermissionDenied => {
                    critical!("Not permitted to read directory `{}`.", path);
                    std::process::exit(1);
                }
                _ => {
                    critical!("Failed to read directory `{}`: {}", path, e);
                    std::process::exit(1);
                }
            }
        }
    }
}
