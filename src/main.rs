use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

mod input;

struct Folder {
    size: i32,
    children: Vec<Folder>,
    name: String,
}

impl Folder {
    fn new(name: String) -> Self {
        Folder {
            name,
            size: 0,
            children: Vec::new(),
        }
    }
    fn update(&mut self) {
        self.children.iter_mut().for_each(|child| {
            child.update();
        });
        self.size += self
            .children
            .iter()
            .fold(0, |total, child| total + child.size);
    }
}

fn main() {
    solve(input::TEST_INPUT.split('\n').collect::<Vec<&str>>());
    solve(input::INPUT.split('\n').collect::<Vec<&str>>())
}

fn solve(commands: Vec<&str>) {
    let mut tree = Folder::new("/".to_string());
    tree = build_tree(tree, commands);
    tree.update();
    println!("{:?}", tree.size);
}

fn build_tree(mut tree: Folder, commands: Vec<&str>) -> Folder {
    let mut dir = vec![];
    commands.iter().for_each(|command| {
        let mut current = &mut tree;
        let split = command.split(' ').collect::<Vec<&str>>();
        if split[1] == "cd" {
            cd(split[2], &mut dir);
            current = navigate(&mut tree, &dir);
        } else if split[0] == "$" || split[0] == "dir" {
            return;
        }
        current.size += add_file(split[0]);
    });
    tree
}

fn cd(directory: &str, path: &mut Vec<String>) {
    if directory == ".." {
        path.pop();
    } else {
        path.push(String::from(directory));
    }
}

fn add_file(line: &str) -> i32 {
    match line.parse::<i32>() {
        Ok(size) => size,
        Err(_err) => 0,
    }
}

fn navigate<'a>(tree: &'a mut Folder, path: &Vec<String>) -> &'a mut Folder {
    let current = tree;
    for dir in path {
        for child in current.children.iter() {
            if child.name == *dir {
                break;
            }
        }
    }
    current
}
