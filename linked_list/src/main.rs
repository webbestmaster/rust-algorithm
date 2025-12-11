#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
}

fn main() {
    println!("Hello, world!");

    let mut node = ListNode::new('a');

    node.next = Some(Box::new(ListNode::new('b')));

    println!("{:?}", dbg!(node.next.unwrap().val));
}
