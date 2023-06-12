use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::time::SystemTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::user::User;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub time: SystemTime,
    pub size: u64,
    pub tokens: Vec<String>,
    pub user: User
}

fn get_file_size(path: &str) -> io::Result<u64> {
    let mut file = fs::File::open(path)?;
    file.rewind()?;
    file.seek(SeekFrom::End(0))
}

pub fn get_info_from_file(ppath: &str, path: &str, user: &User) -> io::Result<FileInfo> {
    let full_path = format!("{}{}{}", ppath, std::path::MAIN_SEPARATOR, path);
    let data = fs::File::open(&full_path)?.metadata()?;
    Ok(FileInfo {
        path: path.replace(std::path::MAIN_SEPARATOR, "/"),
        time: data.modified().unwrap_or(SystemTime::now()),
        size: get_file_size(&full_path)?,
        tokens: vec![],
        user: user.clone()
    })
}

pub fn load_info(path: &str) -> Result<FileInfo, String> {
    match fs::read_to_string(path) {
        Ok(json_str) => {
            match serde_json5::from_str(&json_str) {
                Ok(info) => Ok(info),
                Err(e) => Err(format!("parse json file {} fail: {}", path, e.to_string()))
            }
        }
        Err(e) => Err(format!("read json file {} fail: {}", path, e.to_string()))
    }
}

pub fn save_info(ppath: &str, file_path: &str, info_path: &str, cover: bool, user: &User) -> io::Result<()> {
    match std::path::Path::new(file_path).file_name().unwrap_or(OsStr::new("")).to_str() {
        Some(filename) if filename.len() > 0 => {
            if !cover && fs::metadata(std::path::Path::new(&format!("{}{}{}_info.json", info_path, std::path::MAIN_SEPARATOR, filename))).is_ok() {
                Ok(())
            } else {
                let info = get_info_from_file(ppath, file_path, user)?;
                save_with_info(&info, info_path, cover)
            }
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "invalid file name".to_string()))
    }
}

pub fn save_with_info(fileinfo: &FileInfo, info_path: &str, cover: bool) -> io::Result<()> {
    match std::path::Path::new(&fileinfo.path).file_name().unwrap_or(OsStr::new("")).to_str() {
        Some(filename) if filename.len() > 0 => {
            let info_file = format!("{}{}{}_info.json", info_path, std::path::MAIN_SEPARATOR, filename);
            if !cover && fs::metadata(std::path::Path::new(&info_file)).is_ok() {
                Ok(())
            } else {
                fs::create_dir_all(info_path)?;
                let mut file = fs::OpenOptions::new().write(true).truncate(true).create(true).open(info_file)?;
                match serde_json5::to_string(fileinfo) {
                    Ok(json_str) => match file.write_all(json_str.as_bytes()) {
                        Ok(_) => { file.flush() }
                        Err(e) => Err(e)
                    }
                    Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
                }
            }
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "invalid file name".to_string()))
    }
}

fn gen_info_for_impl(ppath: &str, file_dir: &str, info_path: &std::path::PathBuf, regen: bool, user: &User) -> io::Result<()> {
    let dir = std::fs::read_dir(&format!("{}{}{}", ppath, std::path::MAIN_SEPARATOR, file_dir))?;
    for d in dir {
        let d = d?;
        let filename = d.file_name().to_str().unwrap_or("").to_string();
        if filename.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid filename".to_string()));
        }
        if filename == ".info_dir" {
            continue
        }
        let file_or_dir_path = if file_dir.len() == 0 {
            filename.clone()
        } else {
            format!("{}{}{}", file_dir, std::path::MAIN_SEPARATOR, filename)
        };
        if d.file_type()?.is_file() {
            let info_path = info_path.to_str().unwrap_or("");
            if info_path.len() == 0 {
                return Err(io::Error::new(io::ErrorKind::Other, "invalid filename".to_string()));
            }
            save_info(ppath, &file_or_dir_path, info_path, regen, user)?;
        } else if d.file_type()?.is_dir() {
            gen_info_for_impl(ppath, &file_or_dir_path, &info_path.join(filename), regen, user)?;
        }
    }
    Ok(())
}

pub fn gen_info_for(path: &str, regen: bool, user: &User) -> io::Result<()> {
    let info_root_path = format!("{}{}.info_dir", path, std::path::MAIN_SEPARATOR);
    gen_info_for_impl(path, "", &std::path::Path::new(& info_root_path).to_path_buf(), regen, user)
}

pub fn get_all_info_impl(path: &std::path::Path, container: &mut Vec<FileInfo>) -> io::Result<()> {
    let mut result = Ok(());
    path.read_dir()?.for_each(|d| {
        match d {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) => if file_type.is_file() {
                    match entry.path().to_str() {
                        Some(json_file_path) => {
                            match fs::read_to_string(json_file_path) {
                                Ok(jsonstr) => {
                                    match serde_json5::from_str(&jsonstr) {
                                        Ok(info) => container.push(info),
                                        Err(e) => result = Err(io::Error::new(io::ErrorKind::Other, format!("json to info fail: {}", e.to_string())))
                                    }
                                }
                                Err(e) => result = Err(e)
                            }
                        }
                        None => {
                            result = Err(io::Error::new(io::ErrorKind::Other, "invalid name".to_string()));
                        }
                    }
                } else if file_type.is_dir() {
                    match get_all_info_impl(entry.path().as_path(), container) {
                        Ok(_) => {},
                        Err(e) => eprintln!("get info fail: {}", e.to_string())
                    }
                }
                Err(e) => {
                    result = Err(e);
                }
            }
            Err(e) => {
                result = Err(e);
            }
        }
    });
    result

}

pub fn get_all_info(path: &str) -> io::Result<Vec<FileInfo>> {
    let info_path_str = format!("{}{}.info_dir", path, std::path::MAIN_SEPARATOR);
    let info_path = std::path::Path::new(&info_path_str);
    let mut container = vec![];
    get_all_info_impl(info_path, &mut container)?;
    Ok(container)
}

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use crate::User::UserLAN;
    use super::*;

    #[test]
    fn test_get_info_from_file() {
        let user = UserLAN {
            host_name: "localhost".to_string(),
            ip: "127.0.0.1".to_string()
        };
        let info = get_info_from_file("../../client-core/area_files_client_core/shared_files", "testfile1.txt", &user).unwrap();
        println!("{:?}", info);
    }

    #[test]
    fn test_gen_info_for() {
        let path = "../../client-core/area_files_client_core/shared_files";
        let user = UserLAN {
            host_name: "localhost".to_string(),
            ip: "127.0.0.1".to_string()
        };
        gen_info_for(path, true, &user);
    }

    #[test]
    fn test_get_all_info() {
        let path = "../../client-core/area_files_client_core/shared_files";
        let infos = get_all_info(path).unwrap();
        println!("{:?}", infos);
    }
}
