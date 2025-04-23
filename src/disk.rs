//! Quick file system operations, with pretty logging.

use crate::serror;
use crate as fox;

/// Returns the content of a file as a string.
///
/// Requires valid UTF-8 content.
pub fn read_string(file_path: &str) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            Ok(content)
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    serror!("File `{}` not found.", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to read file `{}`.", file_path);
                }
                std::io::ErrorKind::InvalidData => {
                    serror!("File `{}` is not valid UTF-8.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    serror!("Cannot read `{}`, as it is a directory.", file_path);
                }
                _ => {
                    serror!("Failed to read file `{}`: {}", file_path, e);
                }
            }

            Err(e)
        }
    }
}

/// Returns the bytes of a file.
pub fn read_bytes(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    match std::fs::read(file_path) {
        Ok(content) => {
            Ok(content)
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    serror!("File `{}` not found.", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to read file `{}`.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    serror!("Cannot read `{}`, as it is a directory.", file_path);
                }
                _ => {
                    serror!("Failed to read file `{}`: {}", file_path, e);
                }
            }

            Err(e)
        }
    }
}

/// Writes the given string to a file.
///
/// Overwrites the file if it already exists.
pub fn write_string(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    match std::fs::write(file_path, content) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to write to file `{}`.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    serror!("Cannot write to `{}`, as it is a directory.", file_path);
                }
                _ => {
                    serror!("Failed to write to file `{}`: {}", file_path, e);
                }
            }

            Err(e)
        }
    }
}

/// Writes the given bytes to a file.
///
/// Overwrites the file if it already exists.
pub fn write_bytes(file_path: &str, content: &[u8]) -> Result<(), std::io::Error> {
    match std::fs::write(file_path, content) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to write to file `{}`.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    serror!("Cannot write to `{}`, as it is a directory.", file_path);
                }
                _ => {
                    serror!("Failed to write to file `{}`: {}", file_path, e);
                }
            }

            Err(e)
        }
    }
}

/// Deletes the given file.
pub fn delete_file(file_path: &str) -> Result<(), std::io::Error> {
    match std::fs::remove_file(file_path) {
        Ok(_) => { Ok(()) }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    serror!("File `{}` not found for deletion.", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to delete file `{}`.", file_path);
                }
                std::io::ErrorKind::IsADirectory => {
                    serror!("Cannot delete `{}`, as it is a directory.", file_path);
                }
                _ => {
                    serror!("Failed to delete file `{}`: {}", file_path, e);
                }
            }

            Err(e)
        }
    }
}

/// Reads the metadata of a file.
pub fn file_info(file_path: &str) -> Result<std::fs::Metadata, std::io::Error> {
    match std::fs::metadata(file_path) {
        Ok(metadata) => { Ok(metadata) }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    serror!("File `{}` not found.", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to read metadata of file `{}`.", file_path);
                }
                _ => {
                    serror!("Failed to read metadata of file `{}`: {}", file_path, e);
                }
            }

            Err(e)
        }
    }
}

/// Lists the content of a directory.
pub fn list_dir(path: &str) -> Result<Vec<String>, std::io::Error> {
    match std::fs::read_dir(path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let path_str = path.to_str().unwrap().to_string().strip_prefix("./").unwrap().to_string();

                if path.is_dir() {
                    files.push(format!("{}/", path_str));
                } else {
                    files.push(path_str);
                }

            }

            Ok(files)
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    serror!("Directory `{}` not found.", path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    serror!("Not permitted to read directory `{}`.", path);
                }
                _ => {
                    serror!("Failed to read directory `{}`: {}", path, e);
                }
            }

            Err(e)
        }
    }
}

/// Lists the content of a directory, recursively.
pub fn list_dir_all(path: &str) -> Result<Vec<String>, std::io::Error> {
    fn read_dir_recursive(path: &str, result: &mut Vec<String>) -> Result<(), std::io::Error> {
        match std::fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    let path_str = path.to_str().unwrap().to_string();

                    if path.is_dir() {
                        result.push(format!("{}/", path_str.clone()));
                        read_dir_recursive(&path_str, result)?;
                    } else {
                        result.push(path_str.clone());
                    }
                }
                Ok(())
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        serror!("Directory `{}` not found.", path);
                    }
                    std::io::ErrorKind::PermissionDenied => {
                        serror!("Not permitted to read directory `{}`.", path);
                    }
                    _ => {
                        serror!("Failed to read directory `{}`: {}", path, e);
                    }
                }

                Err(e)
            }
        }
    }

    let mut files = Vec::new();
    read_dir_recursive(path, &mut files)?;

    // Remove the `path` prefix from the paths.
    let path_len = path.len();
    files = files.iter().map(|f| f[path_len..].to_string()).collect();

    Ok(files)
}

