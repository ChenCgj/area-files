use std::collections::HashMap;
use area_files_lib::file_mgr::FileInfo;
use area_files_lib::User;

pub struct AreaFilesData {
    pub my_files: Vec<FileInfo>,
    pub other_files: HashMap<User, Vec<FileInfo>>,
}

impl AreaFilesData {
    pub fn new() -> AreaFilesData {
        AreaFilesData {
            my_files: vec![],
            other_files: HashMap::new(),
        }
    }

    pub fn update_my_files(&mut self, info: &Vec<FileInfo>) {
        self.my_files = info.clone();
    }

    pub fn update_other_files(&mut self, user: &User, info: &Vec<FileInfo>) {
        let mut info = info.clone();
        for info in info.iter_mut() {
            info.user = user.clone();
        }
        *self.other_files.entry(user.clone()).or_insert(vec![]) = info;
    }
}
