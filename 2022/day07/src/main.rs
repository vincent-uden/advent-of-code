use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, repeat, BufRead};
use std::rc::Rc;

#[derive(PartialEq)]
struct TreeNode {
    pub value: Option<String>,
    pub size: Option<usize>,
    pub is_dir: bool,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            value: None,
            size: None,
            is_dir: false,
            children: vec![],
            parent: None,
        };
    }

    pub fn new_with_value(value: String) -> TreeNode {
        return TreeNode {
            value: Some(value),
            size: None,
            is_dir: false,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    pub fn print(&self) -> String {
        return self.internal_print(0);
    }

    fn internal_print(&self, indent: usize) -> String {
        let mut output = String::new();

        let value_disp = match &self.value {
            Some(value) => ("-".repeat(indent) + &value),
            None => String::new(),
        };

        output.push_str(&value_disp);
        if self.is_dir {
            output.push_str(" (dir)");
        }

        match self.size {
            Some(s) => {
                output.push_str(" ");
                output.push_str(&s.to_string())
            }
            None => output.push_str(""),
        }

        for child in &self.children {
            output.push('\n');
            output.push_str(&child.as_ref().borrow().internal_print(indent + 2));
        }

        return output;
    }

    fn calc_sizes(&mut self) -> usize {
        let mut output = match self.size {
            Some(s) => s,
            None => 0,
        };

        for child in &self.children {
            let child_size = child.borrow_mut().calc_sizes();
            output += child_size;
        }

        match self.size {
            Some(_) => {}
            None => self.size = Some(output),
        }
        return output;
    }

    fn sum_size(&self) -> usize {
        let mut output = if self.is_dir {
            match self.size {
                Some(s) => if s <= 100000 { s } else { 0 },
                None => 0,
            }
        } else {
            0
        };

        for child in &self.children {
            let child_size = child.as_ref().borrow().sum_size();
            output += child_size;
        }

        return output;
    }

    fn smallest_delete(&self, size_req: usize) -> Option<usize> {
        let mut smallest = self.size.unwrap();

        for child in &self.children {
            let child_size = match child.as_ref().borrow().smallest_delete(size_req) {
                Some(s) => s,
                None => 0,
            };

            if child_size < smallest && child_size > size_req {
                smallest = child_size;
            }
        }

        return if smallest > size_req {
            Some(smallest)
        } else {
            None
        }
    }
}

fn part_1() -> usize {
    let root = Rc::new(RefCell::new(TreeNode::new_with_value(String::from("/"))));
    root.borrow_mut().is_dir = true;

    let file = File::open("./input.txt").unwrap();

    let mut current = Rc::clone(&root);

    let mut reading_ls = false;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if &l[0..1] == "$" {
                reading_ls = false;
            }

            if reading_ls {
                let current_clone = Rc::clone(&current);

                let parts: Vec<&str> = l.split(" ").collect();
                let new_child = Rc::new(RefCell::new(TreeNode::new_with_value(String::from(
                    parts[1],
                ))));

                if parts[0] == "dir" {
                    new_child.borrow_mut().is_dir = true;
                } else {
                    new_child.borrow_mut().size = Some(parts[0].parse::<usize>().unwrap());
                }
                new_child.borrow_mut().parent = Some(Rc::clone(&current_clone));

                current_clone.borrow_mut().add_child(new_child);
            } else if &l[0..4] == "$ cd" {
                let current_clone = Rc::clone(&current);

                if &l[5..] == ".." {
                    current = Rc::clone(
                        &current_clone
                            .as_ref()
                            .clone()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap(),
                    );
                } else {
                    for i in 0..current_clone.as_ref().borrow().children.len() {
                        let child =
                            Rc::clone(current.as_ref().clone().borrow().children.get(i).unwrap());

                        if child.as_ref().borrow().value.as_ref().unwrap() == &String::from(&l[5..])
                        {
                            current = Rc::clone(&child);
                            break;
                        }
                    }
                }
            } else if &l[0..4] == "$ ls" {
                reading_ls = true;
            }
        }
    }

    root.borrow_mut().calc_sizes();
    println!("{}", root.as_ref().borrow().print());

    let output = root.as_ref().borrow().sum_size();

    return output;
}

fn part_2() -> usize {
    let root = Rc::new(RefCell::new(TreeNode::new_with_value(String::from("/"))));
    root.borrow_mut().is_dir = true;

    let file = File::open("./input.txt").unwrap();

    let mut current = Rc::clone(&root);

    let mut reading_ls = false;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if &l[0..1] == "$" {
                reading_ls = false;
            }

            if reading_ls {
                let current_clone = Rc::clone(&current);

                let parts: Vec<&str> = l.split(" ").collect();
                let new_child = Rc::new(RefCell::new(TreeNode::new_with_value(String::from(
                    parts[1],
                ))));

                if parts[0] == "dir" {
                    new_child.borrow_mut().is_dir = true;
                } else {
                    new_child.borrow_mut().size = Some(parts[0].parse::<usize>().unwrap());
                }
                new_child.borrow_mut().parent = Some(Rc::clone(&current_clone));

                current_clone.borrow_mut().add_child(new_child);
            } else if &l[0..4] == "$ cd" {
                let current_clone = Rc::clone(&current);

                if &l[5..] == ".." {
                    current = Rc::clone(
                        &current_clone
                            .as_ref()
                            .clone()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap(),
                    );
                } else {
                    for i in 0..current_clone.as_ref().borrow().children.len() {
                        let child =
                            Rc::clone(current.as_ref().clone().borrow().children.get(i).unwrap());

                        if child.as_ref().borrow().value.as_ref().unwrap() == &String::from(&l[5..])
                        {
                            current = Rc::clone(&child);
                            break;
                        }
                    }
                }
            } else if &l[0..4] == "$ ls" {
                reading_ls = true;
            }
        }
    }

    root.borrow_mut().calc_sizes();

    let root_clone = Rc::clone(&root);
    let root_size = root_clone.borrow_mut().size.unwrap();

    let output = &root_clone.as_ref().borrow().smallest_delete(30000000-(70000000-root_size));

    return output.unwrap();
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}
