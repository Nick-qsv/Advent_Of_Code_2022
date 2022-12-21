use std::fs;
use std::rc::Rc;

struct TreeNode<T> {
    value: T,
    parent: Option<Rc<TreeNode<T>>>,
    children: Vec<Box<TreeNode<T>>>,
    files: Vec<i32>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Box<TreeNode<T>>) {
        self.children.push(child);
    }

    fn set_parent(&mut self, parent: Rc<TreeNode<T>>) {
        self.parent = Some(parent);
    }
    fn add_file(&mut self, file: i32) {
        self.files.push(file);
    }
}

pub fn day7() {
    let contents = fs::read_to_string("d7input.txt").expect("Failed to read file");
    let mut root = TreeNode::new("/");
    //define the current node, which can be modified
    let mut c_node = &mut root;
    for line in contents.lines() {
        //ls line starts a loop creating and adding children to the current node
        //cd line either goes back up a node or goes into one of the newly created node
        //need to keep track of current node somehow

        //split the command or list into a vec of &str
        let c_line: Vec<&str> = line.split(" ").collect();
        //if it has no $ that means a new file or directory needs to be created
        if c_line[0] != "$" {
            //if its a directory, add a new treenode with value of directory name
            if c_line[0] == "dir" {
                let mut child = Box::new(TreeNode::new(c_line[1]));
                c_node.add_child(child);
                child.set_parent(Rc::new(c_node));
            } else {
                //else add a file to the directory, parsing the initial value into an i32
                c_node.add_file(c_line[0].parse::<i32>().unwrap())
            }
        } else {
            //means there is a "$" so it's a command
            //only command that matters is the "CD" command which means you have to change
            //to a new current node
            // if c_line[2] == ".." {
            //     if let Some(parent) = c_node.parent {
            //         c_node = parent;
            //     }
            // }
        }
    }
    //Feels like you have to make a node structured tree or a DAG
}

//Different commands

//     $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k
// - / (dir)
//   - a (dir)
//     - e (dir)
//       - i (file, size=584)
//     - f (file, size=29116)
//     - g (file, size=2557)
//     - h.lst (file, size=62596)
//   - b.txt (file, size=14848514)
//   - c.dat (file, size=8504156)
//   - d (dir)
//     - j (file, size=4060174)
//     - d.log (file, size=8033020)
//     - d.ext (file, size=5626152)
//     - k (file, size=7214296)
