use std::{
    cell::RefCell,
    env,
    fmt::Display,
    fs,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Debug)]
struct File {
    parent: Rc<RefCell<Dir>>,
    size: u32,
    name: String,
}

impl File {
    pub fn new(name: String, parent: Rc<RefCell<Dir>>, size: u32) -> Self {
        File { parent, size, name }
    }
}

// Dirs might still be in creation, so have to have Option<u32> for size
#[derive(Debug)]
struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
    size: Option<u32>,
    name: String,
}

impl Dir {
    pub fn new(name: String, parent: Option<Rc<RefCell<Dir>>>) -> Self {
        Dir {
            parent,
            subdirs: vec![],
            files: vec![],
            size: None,
            name,
        }
    }

    pub fn create_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Dir::new(String::from("root"), None)))
    }

    pub fn cd(self_: Rc<RefCell<Self>>, name: String) -> Rc<RefCell<Self>> {
        if name == ".." {
            if let Some(parent) = &self_.borrow_mut().parent {
                Rc::clone(&parent)
            } else {
                Rc::clone(&self_)
            }
        } else if name == "/" {
            // keep going up until you find the parent with null
            let mut temp = Rc::clone(&self_);
            //while let Some(parent) = &Dir::cd(temp, "..".to_string()).borrow_mut().parent {
            while let Some(parent) = &temp.clone().borrow_mut().parent {
                if let None = parent.borrow_mut().parent {
                    return Rc::clone(&parent);
                }

                temp = Rc::clone(&parent)
            }

            return Rc::clone(&self_);
        } else {
            for i in self_.borrow_mut().subdirs.iter_mut() {
                if i.borrow_mut().name == name {
                    return Rc::clone(i);
                }
            }

            Rc::new(RefCell::new(Dir::new(name, Some(self_))))
        }
    }

    pub fn read_ls(self_: Rc<RefCell<Self>>, lines: Vec<String>) {
        let mut all_files = true;

        let mut total_size: u32 = 0;
        for line in lines {
            let line: Vec<&str> = line.split(' ').collect();
            if let Ok(size) = line[0].parse::<u32>() {
                self_.borrow_mut().files.push(File::new(
                    String::from(line[1]),
                    Rc::clone(&self_),
                    size,
                ));

                total_size += size;
            } else {
                // must be a directory
                self_
                    .borrow_mut()
                    .subdirs
                    .push(Rc::new(RefCell::new(Dir::new(
                        String::from(line[1]),
                        Some(Rc::clone(&self_)),
                    ))));
                all_files = false;
            }
        }

        if all_files {
            self_.borrow_mut().size = Some(total_size);
        }
    }

    pub fn update_sizes(self_: Rc<RefCell<Self>>) {
        let mut total_size: u32 = 0;
        for i in self_.borrow().subdirs.iter() {
            if let Some(size) = i.borrow().size {
                total_size += size;
                continue;
            }

            Dir::update_sizes(i.clone());
            total_size += i.borrow().size.unwrap();
        }

        let mut total_file_size: u32 = 0;
        for i in self_.borrow().files.iter() {
            total_file_size += i.size;
        }

        self_.borrow_mut().size = Some(total_size + total_file_size);
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut subdirs = String::new();
        for i in self.subdirs.clone() {
            let i = i.borrow();
            subdirs += &i.to_string();
        }

        let mut files = String::new();
        for i in self.files.iter() {
            files += &format!("file_name: {}, size: {}\n", i.name, i.size).to_string();
        }

        f.write_str(
            format!(
                "name: {}, size: {:?}, subdirs 
[
--{}
], files [
--{}
]",
                self.name, self.size, subdirs, files
            )
            .as_str(),
        )
    }
}

fn main() {
    let file = fs::File::open(env::args().nth(1).unwrap()).unwrap();
    let lines = BufReader::new(file).lines();

    let root = Dir::create_root();
    let mut temp = Rc::clone(&root);
    let mut reading_ls = false;
    let mut ls_vec: Vec<String> = vec![];
    for line in lines {
        if let Err(_) = line {
            break;
        }
        let line = line.unwrap().clone();
        if reading_ls {
            // if the first char is '$', meaning it is a command stop reading ls
            if line.chars().next().unwrap() == '$' {
                reading_ls = false;
                Dir::read_ls(temp.clone(), ls_vec.clone());
            } else {
                ls_vec.push(line);
                continue;
            }
        }
        let line: Vec<&str> = line.split(" ").collect();

        // first read everything
        // todo: finish the ls thing
        //  after the reading do udpate and maybe everything will work
        if line[0] == "$" {
            if line[1] == "cd" {
                temp = Dir::cd(temp.clone(), String::from(line[2]));
            } else {
                // line[1] == "ls"
                // super hacky but wtaver
                // go into "reading ls" mode for the next couple of iterations, if we see a command
                // then stop and read everything
                ls_vec.clear();
                reading_ls = true;
            }
        }
    }

    if reading_ls {
        Dir::read_ls(temp.clone(), ls_vec);
    }

    Dir::update_sizes(root.clone());

    // todo: get the total sizes thing
    /*println!(
        "{}\n{}: {}",
        root.clone().borrow(),
        root.clone().borrow().name,
        root.clone().borrow().size.unwrap()
    );*/

    // go through every directory, if it has total_size < 10000 add its size
    println!("{}", get_total_less_than_100k(root.clone()));
}

fn get_total_less_than_100k(root: Rc<RefCell<Dir>>) -> u32 {
    let mut total: u32 = 0;
    let cur_size = root.borrow().size.unwrap();
    if cur_size <= 100000 {
        total += cur_size;
    }

    for i in root.borrow().subdirs.iter() {
        total += get_total_less_than_100k(i.clone());
    }

    total
}
