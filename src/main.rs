use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

mod input;

#[derive(Debug)]
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
    println!("\n");
    solve(input::INPUT.split('\n').collect::<Vec<&str>>())
}

fn solve(commands: Vec<&str>) {
    let mut tree = Folder::new("/".to_string());
    tree = build_tree(tree, commands);
    tree.update();
    println!("{:?}", tree);
}

fn build_tree(mut tree: Folder, commands: Vec<&str>) -> Folder {
    let mut dir = vec![];
    commands.iter().for_each(|command| {
        let split = command.split(' ').collect::<Vec<&str>>();
        if split[1] == "cd" && split[2] != "/" {
            cd(split[2], &mut dir);
        } else if split[0] == "$" {
            return;
        } else if split[0] == "dir" {
            let current = navigate(&mut tree, &dir);
            let name_later = current.children.iter().find(|x| x.name == split[1]);
            if name_later.is_none() {
                current.children.push(Folder::new(split[1].to_string()))
            }
        } else {
            let current = navigate(&mut tree, &dir);
            current.size += add_file(split[0]);
        }
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

fn navigate<'a>(tree: &'a mut Folder, path: &[String]) -> &'a mut Folder {
    let result = path.iter().fold(tree, |curr, dir| {
        let sub_folder = curr
            .children
            .iter_mut()
            .find(|subfolder| subfolder.name == *dir);
        match sub_folder {
            Some(sub_folder) => sub_folder,
            None => panic!("folder not found"),
        }
    });
    result
}
