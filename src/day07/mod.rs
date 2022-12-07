mod input;

use std::collections::HashMap;

type CurrentDir = Vec<String>;
type Directories = HashMap<String, usize>;

fn cd(current_directory: CurrentDir, arg: &str) -> CurrentDir {
    let mut current_directory = current_directory.clone();

    if arg == ".." {
        current_directory.pop();
    } else {
        current_directory.push(arg.to_string());
    }

    return current_directory;
}

fn dir(directories: Directories, current_directory: &CurrentDir, arg: &str) -> Directories {
    let mut directories = directories.clone();
    let mut directory = path(current_directory);
    if !directory.ends_with("/") {
        directory.push_str("/");
    }
    directory.push_str(arg);
    directories.insert(directory, 0);
    return directories;
}

fn path(current_directory: &CurrentDir) -> String {
    if current_directory.len() > 1 {
        return ["/".to_string(), current_directory[1..].join("/")].concat();
    }
    current_directory.join("/")
}

fn file(
    directories: &Directories,
    current_directory: &CurrentDir,
    filesize: usize
) -> Directories {
    let mut directories = directories.clone();
    let path = path(current_directory);
    let dirsize = directories[&path].clone();
    directories.insert(path, dirsize + filesize);
    return directories;
}

fn dir_size(directories: &Directories, path: &String) -> usize {
    directories
        .keys()
        .filter(|p| p.starts_with(path))
        .map(|p| directories[p])
        .sum()
}

fn part1(input: &str) -> usize {
    let lines = input.split('\n');

    let mut directories: Directories = HashMap::new();
    directories.insert("/".to_string(), 0);
    let mut current_directory: CurrentDir = vec![];

    for line in lines {
        let line = line.replace("$ ", "");
        let (cmd, arg) = match line.split_once(" ") {
            Some((cmd, arg)) => (cmd, arg),
            None => ("", "")
        };

        match cmd {
            "cd" => current_directory = cd(current_directory, arg),
            "dir" => directories = dir(directories, &current_directory, arg),
            "ls" => (),
            "" => (),
            filesize => directories = file(
                &directories,
                &current_directory,
                filesize.parse::<usize>().unwrap()
            )
        }
    }

    let total_size: usize = directories
        .keys()
        .map(|path| dir_size(&directories, &path))
        .filter(|size| *size < 100e3 as usize)
        .sum();

    total_size
}

pub fn test() {
    assert_eq!(part1(input::TEST), 95437);
    assert_eq!(part1(input::REAL), 0);
}
