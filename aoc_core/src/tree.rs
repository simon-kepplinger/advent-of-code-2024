use std::collections::VecDeque;

pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, node: TreeNode<T>) {
        self.children.push(node);
    }
}

pub struct TreeRoot<T> {
    pub root_node: Option<TreeNode<T>>,
}

impl<T: Copy> TreeRoot<T> {
    pub fn new(node: Option<TreeNode<T>>) -> Self {
        TreeRoot { root_node: node }
    }

    pub fn push_deep(&mut self, node: TreeNode<T>) {
        match self.root_node {
            Some(ref mut root_node) => {
                let mut current = root_node;
                while current.children.len() > 0 {
                    current = &mut current.children[0];
                }
                current.push(node);
            }
            None => self.root_node = Some(node),
        }
    }

    pub fn has_nodes(&self) -> bool {
        self.root_node.is_some()
    }

    pub fn iter_depth(&self) -> TreeDepthIterator<T> {
        TreeDepthIterator {
            stack: self.root_node.iter().collect(),
        }
    }

    pub fn iter_depth_rev(
        &self,
    ) -> RevereseTreeIterator<T, TreeDepthIterator<T>> {
        RevereseTreeIterator {
            buffer: None,
            iter: self.iter_depth(),
        }
    }

    pub fn iter_breadth(&self) -> TreeBreadthIterator<T> {
        TreeBreadthIterator {
            queue: self.root_node.iter().collect(),
        }
    }

    pub fn iter_breadth_rev(
        &self,
    ) -> RevereseTreeIterator<T, TreeBreadthIterator<T>> {
        RevereseTreeIterator {
            buffer: None,
            iter: self.iter_breadth(),
        }
    }
}

pub struct TreeDepthIterator<'a, T> {
    stack: Vec<&'a TreeNode<T>>,
}

impl<'a, T> Iterator for TreeDepthIterator<'a, T> {
    type Item = &'a TreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(node) => {
                for child in node.children.iter().rev() {
                    self.stack.push(child);
                }

                Some(node)
            }
            None => None,
        }
    }
}

pub struct TreeBreadthIterator<'a, T> {
    queue: VecDeque<&'a TreeNode<T>>,
}

impl<'a, T> Iterator for TreeBreadthIterator<'a, T> {
    type Item = &'a TreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(node) => {
                for child in node.children.iter() {
                    self.queue.push_back(&child);
                }

                Some(node)
            }
            None => None,
        }
    }
}

// this is a simple reverse iterator. A much better approach would be to implement
// `DoubleEndedIterator` for the `TreeDepthIterator`
pub struct RevereseTreeIterator<'a, T, I>
where
    I: Iterator<Item = &'a TreeNode<T>>,
{
    buffer: Option<Vec<&'a TreeNode<T>>>,
    iter: I,
}

impl<'a, T, I> Iterator for RevereseTreeIterator<'a, T, I>
where
    I: Iterator<Item = &'a TreeNode<T>>,
{
    type Item = &'a TreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer {
            Some(ref mut buffer) => buffer.pop(),
            None => {
                self.buffer = Some((&mut self.iter).collect());
                self.next()
            }
        }
    }
}
