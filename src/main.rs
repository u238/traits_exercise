mod functions;

use std::thread;
use std::sync::mpsc::channel;

type LinkedList = Option<Node>;

struct Node {
    val: i64,
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

trait Map {
    fn mapr(&mut self, fn(i64) -> i64);
}

impl Length for LinkedList {
    fn length(&self) -> i32 {
        match self {
            &Some(ref node) => { 1 + node.tail.length() }
            &None => 0
        }
    }
}

fn construct_list(n: i64, x: i64) -> LinkedList {
    match n {
        0 => { None }
        _ => { Some(Node{val: x, tail: Box::new(construct_list(n - 1, x + 1))}) }
    }
}

fn print_list(p: LinkedList) {
    for n in p.iter() {
        println!("{}, ", n.val);
    }
}

impl Map for LinkedList {
    fn mapr(&mut self, f: fn(i64) -> i64) {
        match self {
            None => { }
            Some(ref mut current) => {
                let (tx, rx) = channel();
                let val = current.val; // Can't capture current
                thread::spawn(move || { tx.send(f(val)).expect("error sending to channel"); });
                current.tail.mapr(f); // why here?
                current.val = rx.recv().expect("error receiving from channel");
            }
        }
    }
}

fn expensive_inc(n: i64) -> i64 {
    let mut a: i64 = 1;
    println!("starting inc: {}", n);
    for _ in 0..1000000000 {
        for _ in 0..1000000000 {
            a = a + 6;
        }
    }

    println!("finished inc: {} ({:?})", n, a);
    n + 1
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
                        &Some(ref _node) => {
                            //let mut buf = String::new();
//                            buf += node.to_string()
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
    println!("tree: {}", tree.to_string());

    functions::test_fn();
    let sixthpower = functions::compose(functions::cube, functions::square);
    println!("sixtpower(4): {}", sixthpower(3));

    let mut p : LinkedList = construct_list(5, 10);
    p.mapr(expensive_inc);
    print_list(p);
}
