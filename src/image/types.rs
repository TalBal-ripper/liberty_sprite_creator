use std::path::PathBuf;

#[derive(Debug)]
pub struct ParentDir {
    pub name: String,
    pub children: Vec<ChildDir>,
}

#[derive(Debug)]
pub struct ChildDir {
    pub name: String,
    pub files: Vec<PathBuf>,
}
