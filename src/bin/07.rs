use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct FileSystem {
    root: Rc<Directory>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}
#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<Directory>>,
    subdir: RefCell<HashMap<String, Rc<Directory>>>,
    files: RefCell<HashMap<String, Rc<File>>>,
}

impl Directory {
    fn new(name: &str, parent: Option<Rc<Directory>>) -> Self {
        Directory {
            name: name.to_owned(),
            subdir: RefCell::new(HashMap::new()),
            files: RefCell::new(HashMap::new()),
            parent,
        }
    }

    fn get_subdirs(&self) -> Vec<Rc<Directory>> {
        self.subdir
            .borrow()
            .values()
            .into_iter()
            .map(|subdir| {
                let mut vec = Vec::new();
                vec.push(Rc::clone(subdir));
                vec.append(subdir.get_subdirs().as_mut());
                vec
            })
            .flatten()
            .collect()
    }
}

impl FileSystem {
    fn new() -> Self {
        let root = Directory::new("/", None);
        FileSystem {
            root: Rc::new(root),
        }
    }

    fn parse_input(&mut self, input: &str) -> &Self {
        let lines = input.lines();
        let mut pwd = Rc::clone(&self.root);

        for line in lines {
            let words = line.split(' ').collect::<Vec<&str>>();
            match (words[0], words[1]) {
                ("$", "ls") => {}
                ("$", "cd") => {
                    pwd = match words[2] {
                        "/" => Rc::clone(&self.root),
                        ".." => Rc::clone(&pwd.parent.as_ref().unwrap()),
                        dirname => pwd.subdir.borrow().get(dirname).unwrap().clone(),
                    };
                }
                ("dir", dirname) => {
                    if pwd.subdir.borrow().get(dirname).is_none() {
                        pwd.subdir.borrow_mut().insert(
                            dirname.to_owned(),
                            Rc::new(Directory::new(dirname, Some(Rc::clone(&pwd)))),
                        );
                    } else {
                        ()
                    }
                }
                (size, filename) => {
                    if pwd.files.borrow().get(filename).is_none() {
                        pwd.files.borrow_mut().insert(
                            filename.to_owned(),
                            Rc::new(File {
                                name: filename.to_owned(),
                                size: size.parse().unwrap(),
                            }),
                        );
                    } else {
                        ()
                    }
                }
            }
        }
        self
    }

    fn get_dir_size(dir: &Directory) -> usize {
        let direct: usize = dir
            .files
            .borrow()
            .values()
            .into_iter()
            .map(|file| file.size)
            .sum();
        let children: usize = dir
            .subdir
            .borrow()
            .values()
            .into_iter()
            .map(|subdir| FileSystem::get_dir_size(subdir))
            .sum();
        direct + children
    }

    fn get_dirs(&self) -> Vec<Rc<Directory>> {
        self.root.get_subdirs()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut file_system = FileSystem::new();
    file_system.parse_input(input);
    let sized_dirs: usize = file_system
        .get_dirs()
        .into_iter()
        .map(|dir| FileSystem::get_dir_size(dir.borrow()))
        .filter(|size| *size < 100000)
        .sum();
    Some(sized_dirs as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut file_system = FileSystem::new();
    file_system.parse_input(input);
    let used_disk_space = FileSystem::get_dir_size(file_system.root.as_ref());
    println!("{}", used_disk_space);
    let free_space = 70000000 - used_disk_space;

    let smallest_valid_size = file_system
        .get_dirs()
        .into_iter()
        .map(|dir| FileSystem::get_dir_size(dir.borrow()))
        .filter(|dir_size| {
            // total disk free = free_space
            // needed_free = 30000000
            free_space + dir_size >= 30000000
        })
        .min()
        .unwrap();
    Some(smallest_valid_size as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
