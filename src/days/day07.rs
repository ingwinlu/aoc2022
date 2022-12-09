use std::{cell::RefCell, fmt, rc::Rc};

use crate::{Solution, SolutionPair};

type Node = Rc<RefCell<Folder>>;

struct Folder {
    name: String,
    parent: Option<Node>,
    children: Vec<Box<FolderChild>>,
}

impl Folder {
    fn new(name: &str, parent: Option<Node>) -> Node {
        Rc::new(RefCell::new(Self {
            name: name.to_string(),
            children: Vec::new(),
            parent,
        }))
    }

    fn add_child(self: &mut Self, child: FolderChild) {
        self.children.push(Box::new(child));
    }

    fn size(self: &Self) -> u64 {
        self.children.iter().fold(0, |acc, child| {
            // wat
            acc + match &**child {
                FolderChild::File(file) => file.size,
                FolderChild::Folder(folder) => folder.borrow().size(),
            }
        })
    }

    fn size_list(self: &Self) -> Vec<(String, u64)> {
        let current = (self.name.clone(), self.size());

        let mut others: Vec<(String, u64)> = self
            .children
            .iter()
            .filter_map(|child| match &**child {
                FolderChild::Folder(folder) => Some(folder.borrow().size_list()),
                _ => None,
            })
            .flatten()
            .collect();

        others.push(current);
        others
    }
}

impl fmt::Debug for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Folder")
            .field("name", &self.name)
            .field("children", &self.children)
            .finish()
    }
}

struct File {
    name: String,
    size: u64,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Folder")
            .field("name", &self.name)
            .field("size", &self.size)
            .finish()
    }
}

impl File {
    fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

enum FolderChild {
    File(File),
    Folder(Node),
}

impl fmt::Debug for FolderChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FolderChild::File(file) => f.debug_struct("File").field("content", &file).finish(),
            FolderChild::Folder(folder) => {
                f.debug_struct("Folder").field("content", &folder).finish()
            }
        }
    }
}

fn parse(input: &str) -> Node {
    let lines = input.lines();
    let root = Folder::new("/", None);
    let mut current = root.clone();
    let mut parse_ls = false;
    for line in lines {
        if line.starts_with("$") {
            parse_ls = false;
        }
        match line {
            "$ cd /" => {}
            "$ cd .." => {
                let next_current = current.borrow_mut().parent.take().unwrap();
                current = next_current;
            }
            _ if line.starts_with("$ cd ") => {
                // bug here when doing $ cd .., would need to track current path
                let new_dir = &line[5..];
                let new_folder = Folder::new(new_dir, Some(current.clone()));
                current
                    .borrow_mut()
                    .add_child(FolderChild::Folder(new_folder.clone()));
                current = new_folder.clone();
            }
            "$ ls" => {
                parse_ls = true;
            }
            _ if parse_ls && line.starts_with("dir") => {
                let (_, _new_dir_name) = line.split_once(" ").unwrap();
                // do not need to do anything with this yet
            }
            _ if parse_ls => {
                let (size, file_name) = line.split_once(" ").unwrap();
                let file = File::new(file_name, size.parse().unwrap());
                current.borrow_mut().add_child(FolderChild::File(file));
            }
            _ => unreachable!(),
        }
        //println!("{:?}", root);
    }
    root
}

fn solve_day1(input: &str) -> u64 {
    let out = parse(input);
    // println!("{out:?}");
    // println!("{}", out.borrow().size());
    // println!("{:?}", out.borrow().size_list());

    let correct: u64 = out
        .borrow()
        .size_list()
        .into_iter()
        .filter(|ele| ele.1 < 100_000)
        .map(|(_name, size)| size)
        .sum();
    correct
}

fn solve_day2(input: &str) -> u64 {
    let out = parse(input);
    let total_size = 70_000_000u64;
    let currently_used = out.borrow().size();
    let currently_free = total_size - currently_used;

    let update_size_required = 30000000;
    let free_up_space_for_update = update_size_required - currently_free;

    let correct: u64 = out
        .borrow()
        .size_list()
        .into_iter()
        .filter(|ele| ele.1 > free_up_space_for_update)
        .map(|(_name, size)| size)
        .min()
        .unwrap();
    correct
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day07.txt");
    let sol1 = solve_day1(&input);
    let sol2 = solve_day2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_day_1() {
        let input = indoc! {"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "};
        let result = solve_day1(input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn sample_day_2() {
        let input = indoc! {"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "};
        let result = solve_day2(input);
        assert_eq!(result, 24933642);
    }
}
