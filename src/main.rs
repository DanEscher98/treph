use serde::{Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Component, Components, Path};

#[derive(Debug, Serialize)]
struct TreeNodeWrapper<'a>(RefCell<TreeNode<'a>>);

impl TreeNodeWrapper<'_> {
    fn name(&self) -> Component {
        self.0.borrow().name
    }
}

impl<'a> PartialEq for TreeNodeWrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for TreeNodeWrapper<'_> {}

impl Hash for TreeNodeWrapper<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

fn serialize_component<S>(component: &Component, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&component.as_os_str().to_string_lossy())
}

#[derive(Debug, Eq, PartialEq, Serialize)]
struct TreeNode<'a> {
    #[serde(serialize_with = "serialize_component")]
    name: Component<'a>,
    level: usize,
    children: HashSet<TreeNodeWrapper<'a>>,
}

impl Hash for TreeNode<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> TreeNode<'a> {
    fn new(name: Component<'a>, level: usize) -> Self {
        Self {
            name,
            level,
            children: Default::default(),
        }
    }
    fn add_path(&mut self, components: &mut Components<'a>) {
        if let Some(part) = components.next() {
            if let Some(node) = self.children.iter().find(|&node| node.name() == part) {
                node.0.borrow_mut().add_path(components);
            } else {
                let new_node = TreeNodeWrapper(RefCell::new(TreeNode::new(part, self.level + 1)));
                new_node.0.borrow_mut().add_path(components);
                self.children.insert(new_node);
            }
        }
    }
    fn print_tree(&self) {
        if self.level > 0 {
            let indentation = "  ".repeat(self.level - 1);
            println!("{}{}", indentation, self.name.as_os_str().to_string_lossy());
        }
        for child in &self.children {
            child.0.borrow().print_tree();
        }
    }
}

fn main() {
    let mut cache = TreeNode::new(Component::RootDir, 0);

    let paths = [
        "/usr/bin/bash",
        "workdir/bin/bash",
        "/home/rusty/hello.txt",
        "/home/rusty/dummy.txt",
        "/usr/bin/python",
        "/home/ellie/workdir/setup.sh",
        "/usr/bin/bash",
    ];

    for path in paths {
        cache.add_path(&mut Path::new(path).components());
    }
    cache.print_tree();
}
