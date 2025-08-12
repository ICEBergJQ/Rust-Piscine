#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Node {
            value,
            next: None,
        };

        match self.head.take() {
            None => self.head = Some(new_node),
            Some(existing_head) => {
                new_node.next = Some(Box::new(existing_head));
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop(&mut self) {
        if let Some(current_head) = self.head.take() {
            self.head = current_head.next.map(|boxed_node| *boxed_node);
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current_node_ref = self.head.as_ref();

        while let Some(node) = current_node_ref {
            length += 1;
            current_node_ref = node.next.as_ref().map(|boxed| &**boxed);
        }

        length
    }
}
