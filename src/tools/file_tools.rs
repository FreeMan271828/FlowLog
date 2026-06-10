use std::{fs::{self, File}, io::{self, Write}, path::PathBuf};

use serde::Serialize;

/// 写入指定的文件
pub fn write_to_file<T: Serialize>(file_path: &PathBuf, data: &T) -> Result<(), io::Error>{
    let file = fs::File::create(file_path)?;
    write_to_file_(&file, data)
}
pub fn write_to_file_<T: Serialize>(mut file: &File, data: &T) -> Result<(), io::Error>{
    let json = serde_json::to_string(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    file.write_all(json.as_bytes()).expect("Write File Err");
    file.flush().expect("Flush File Err");
    Ok(())
}

/// 获取中指定的文件大小
pub fn get_file_size(file_path: &PathBuf) -> Result<u64, io::Error>{
    let meta = fs::metadata(&file_path)?;
    return Ok(meta.len());
}

/// 获取一个文件夹中的最老的文件
pub fn get_oldest_file(dir_path: &PathBuf) -> Result<Option<PathBuf>, std::io::Error> {
    let files: Vec<PathBuf> = get_sorted_files(dir_path)?;
    if files.is_empty() {
        Ok(None)
    } else {
        let oldest = files.first().unwrap().clone();
        Ok(Some(oldest))
    }
}

/// 获取一个文件夹中的最新的文件
pub fn get_newest_file(dir_path: &PathBuf) -> Result<Option<PathBuf>, std::io::Error> {
    let files: Vec<PathBuf> = get_sorted_files(dir_path)?;
    if files.is_empty() {
        Ok(None)
    } else {
        let oldest = files.last().unwrap().clone();
        Ok(Some(oldest))
    }
}

/// 按照时间排序文件夹中的文件
pub fn get_sorted_files(dir_path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files: Vec<PathBuf> = dir_path.read_dir()?
        .filter_map(|entry|  {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name()?.to_str() {
                    if file_name.starts_with("log-") {
                        return Some(path);
                    }
                }
            }
            None
        }
        )
        .collect();
    files.sort();
    Ok(files)
}