use std::collections::HashSet;
use std::fs;
use std::io;
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

pub fn get_path() -> io::Result<Vec<ParentDir>> {
    let mut result = Vec::new();

    for parent in fs::read_dir("./src/img")? {
        let parent = parent?;
        let parent_path = parent.path();

        if !parent_path.is_dir() {
            continue;
        }

        let parent_name = parent_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let mut children = Vec::new();

        for child in fs::read_dir(&parent_path)? {
            let child = child?;
            let child_path = child.path();

            if !child_path.is_dir() {
                continue;
            }

            let child_name = child_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let mut files = Vec::new();

            for file in fs::read_dir(&child_path)? {
                let file = file?;
                let file_path = file.path();

                if file_path.is_file() {
                    files.push(file_path);
                }
            }

            children.push(ChildDir {
                name: child_name,
                files,
            });
        }

        result.push(ParentDir {
            name: parent_name,
            children,
        });
    }
    Ok(result)
}

pub fn get_childrens() -> Result<Vec<String>, std::io::Error> {
    let data = get_path()?;
    Ok(data
        .iter()
        .flat_map(|p| &p.children)
        .map(|c| c.name.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}

pub fn get_parents() -> Result<Vec<String>, std::io::Error> {
    let mut parents: Vec<String> = get_path()?.iter().map(|p| p.name.clone()).collect();
    parents.sort();
    Ok(parents)
}

pub fn get_img() -> Result<Vec<String>, std::io::Error> {
    let data = get_path()?;

    Ok(data
        .iter()
        .flat_map(|p| &p.children)
        .flat_map(|c| &c.files)
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}
