#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) next: Option<*mut Node<T>>,
    pub(crate) prev: Option<*mut Node<T>>,
    pub(crate) val: T,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct LinkedList<T> {
    pub(crate) head: Option<*mut Node<T>>,
    pub(crate) tail: Option<*mut Node<T>>,
    pub(crate) len: usize,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_tail().is_some() {}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_tail(&mut self, val: T) {
        let node = Box::new(Node {
            prev: self.tail,
            next: None,
            val,
        });
        let node = Box::leak(node);

        let node_ptr = node as *mut Node<T>;

        if let Some(tail_ptr) = self.tail {
            unsafe {
                (*tail_ptr).next = Some(node_ptr);
            }
        }
        self.tail = Some(node_ptr);

        if self.head.is_none() {
            self.head = Some(node_ptr);
        }

        self.len += 1;
    }

    pub fn push_head(&mut self, val: T) {
        let node = Box::new(Node {
            prev: None,
            next: self.head,
            val,
        });
        let node = Box::leak(node);

        let node_ptr = node as *mut Node<T>;

        if let Some(head_ptr) = self.head {
            unsafe {
                (*head_ptr).prev = Some(node_ptr);
            }
        }
        self.head = Some(node_ptr);

        if self.tail.is_none() {
            self.tail = Some(node_ptr);
        }

        self.len += 1;
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        let tail_ptr = self.tail?;
        let node = unsafe { Box::from_raw(tail_ptr) };

        match node.prev {
            Some(prev_ptr) => unsafe {
                (*prev_ptr).next = None;
            },
            None => self.head = None,
        }

        self.tail = node.prev;

        self.len -= 1;

        Some(node.val)
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let head_ptr = self.head?;
        let node = unsafe { Box::from_raw(head_ptr) };

        match node.next {
            Some(next_ptr) => unsafe {
                (*next_ptr).prev = None;
            },
            None => self.tail = None,
        }

        self.head = node.next;

        self.len -= 1;

        Some(node.val)
    }
}
