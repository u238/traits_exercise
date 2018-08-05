type LinkedList = Option<Node>;

struct Node {
    val: i32,
    tail: Box<LinkedList>,
}

type Tree = Option<TreeNode>;

struct TreeNode {
    val: i32,
    children: Box<Vec<Tree>>
}


trait Length {
    fn length(&self) -> i32;
}

trait ToString {
    fn to_string(&self) -> String;
}

impl Length for LinkedList {
    fn length(&self) -> i32 {
        match self {
            &Some(ref node) => { 1 + node.tail.length() }
            &None => 0
        }
    }
}

impl ToString for LinkedList {
    fn to_string(&self) -> String {
        match self {
            &Some(ref node) => {

                let mut buf = String::new();
                buf += &format!("[ {}, {} ]", node.val, node.tail.to_string());
                buf

            }
            &None => "()".to_owned()
        }
    }
}

impl ToString for Tree {
    fn to_string(&self) -> String {
        match self {
            &Some(ref node) => {
                let mut buf = String::new();
                buf += &format!("[ {}, ", node.val);
                for c in node.children.iter() {
                    match c {
                        &Some(ref node) => {
                            //let mut buf = String::new();
                            buf += node.to_string()
                        }
                        &None => {
                            buf += "()"
                        }
                    }
                }
                buf
            }
            &None => "()".to_owned()
        }
    }
}

fn main() {

    let ll: LinkedList = Some(Node{val:1, tail: Box::new(Some(Node{val: 2, tail: Box::new(None)}))});

    let tree: Tree = Some(TreeNode{val:47, children: Box::new(vec![])});

    println!("linkedlist: {}", ll.to_string());
    println!("linkedlist: {}", tree.to_string());
}
