use std::{cell::RefCell, rc::Rc, rc::Weak, str::FromStr};

#[derive(Debug)]
enum Command {
    Ls,
    CdIn { name: String },
    CdOut,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
}

#[derive(Debug)]
enum Content {
    File(File),
    Dir(Dir),
}

#[derive(Debug)]
enum ParsedLine {
    Command(Command),
    Content(Content),
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
struct Node {
    name: String,
    parent: Weak<Node>,
    files: RefCell<Vec<File>>,
    dirs: RefCell<Vec<Dir>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "$ ls" {
            Ok(Command::Ls)
        } else if s == "$ cd .." {
            Ok(Command::CdOut)
        } else if s.starts_with("$ cd ") {
            let name = s.split(" ").take(3).last().ok_or(ParseError)?.into();
            Ok(Command::CdIn { name })
        } else {
            Err(ParseError)
        }
    }
}

impl FromStr for Content {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir ") {
            let name = s.split_once(" ").ok_or(ParseError)?.1.into();
            Ok(Content::Dir(Dir { name }))
        } else {
            let (size_str, name) = s.split_once(" ").ok_or(ParseError)?;
            let size = size_str.parse::<usize>().ok().ok_or(ParseError)?;
            Ok(Content::File(File {
                name: name.to_string(),
                size,
            }))
        }
    }
}

impl FromStr for ParsedLine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('$') {
            let command = s.parse::<Command>()?;
            Ok(ParsedLine::Command(command))
        } else {
            let content = s.parse::<Content>()?;
            Ok(ParsedLine::Content(content))
        }
    }
}

fn dir_size(node: &Node) -> usize {
    let file_size: usize = node.files.borrow().iter().map(|f| f.size).sum();
    let dir_size: usize = node
        .children
        .borrow()
        .iter()
        .map(|d| dir_size(d.as_ref()))
        .sum();
    file_size + dir_size
}

struct NodeIter {
    node_stack: Vec<Rc<Node>>,
}

impl Iterator for NodeIter {
    type Item = Rc<Node>;
    fn next(&mut self) -> Option<Self::Item> {
        let maybe_item = self.node_stack.pop();

        if let Some(item) = &maybe_item {
            self.node_stack
                .extend(item.children.borrow().iter().rev().cloned());
        }
        return maybe_item;
    }
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(&arg).unwrap();

    let input = file_content.split('\n').filter(|&s| !s.is_empty()).skip(1);
    let mut commands: Vec<(Command, Vec<File>, Vec<Dir>)> = vec![];
    for line in input {
        let parsed_line = line.parse::<ParsedLine>().unwrap();
        match parsed_line {
            ParsedLine::Command(command) => commands.push((command, vec![], vec![])),
            ParsedLine::Content(Content::File(file)) => commands.last_mut().unwrap().1.push(file),
            ParsedLine::Content(Content::Dir(dir)) => commands.last_mut().unwrap().2.push(dir),
        }
    }

    let tree = Rc::new(Node {
        name: "/".into(),
        parent: Weak::new(),
        files: RefCell::new(vec![]),
        dirs: RefCell::new(vec![]),
        children: RefCell::new(vec![]),
    });

    let mut current_node = Rc::downgrade(&tree);

    for cmd in commands {
        //println!("{:?}", cmd);
        match cmd {
            (Command::Ls, files, dirs) => {
                current_node
                    .upgrade()
                    .unwrap()
                    .files
                    .borrow_mut()
                    .extend(files.iter().cloned());
                current_node
                    .upgrade()
                    .unwrap()
                    .dirs
                    .borrow_mut()
                    .extend(dirs.iter().cloned());
            }
            (Command::CdIn { name }, _, _) => {
                let child_node = Rc::new(Node {
                    name,
                    parent: Weak::clone(&current_node),
                    files: RefCell::new(vec![]),
                    dirs: RefCell::new(vec![]),
                    children: RefCell::new(vec![]),
                });
                current_node
                    .upgrade()
                    .unwrap()
                    .children
                    .borrow_mut()
                    .push(Rc::clone(&child_node));
                current_node = Rc::downgrade(&child_node);
            }
            (Command::CdOut, _, _) => {
                current_node = current_node.upgrade().unwrap().parent.clone();
            }
        }
    }

    let available: usize = 70000000;
    let used = dir_size(tree.as_ref());
    let required: usize = 30000000;
    let unused = available - used;
    let space_needed = required - unused;
    let node_iter = NodeIter {
        node_stack: vec![tree.clone()],
    };
    let result: usize = node_iter
        .map(|node| dir_size(node.as_ref()))
        .filter(|&node_size| node_size >= space_needed)
        .min()
        .unwrap();
    println!("{}", result)
}
