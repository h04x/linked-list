use crate::LinkedList;

#[test]
fn push_tail() {
    let mut l = LinkedList::new();
    assert_eq!(
        l,
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    );

    l.push_tail(333);
    assert_eq!(l.len(), 1);
    assert_eq!(l.head, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    l.push_tail(444);
    assert_eq!(l.len(), 2);
    assert_eq!(unsafe { (*l.head.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 444);
    assert_eq!(unsafe { (*l.head.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.head.unwrap()).next }, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, l.head);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    l.push_tail(555);
    assert_eq!(l.len(), 3);
    assert_eq!(unsafe { (*l.head.unwrap()).val }, 333);
    assert_eq!(unsafe { (*(*l.head.unwrap()).next.unwrap()).val }, 444);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 555);
    assert_eq!(unsafe { (*l.head.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.head.unwrap()).next }, unsafe {
        (*l.tail.unwrap()).prev
    });
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);
    assert_eq!(unsafe { (*(*l.head.unwrap()).next.unwrap()).prev }, l.head);
    assert_eq!(unsafe { (*(*l.head.unwrap()).next.unwrap()).next }, l.tail);
}

#[test]
fn push_head() {
    let mut l = LinkedList::new();
    assert_eq!(
        l,
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    );

    l.push_head(333);
    assert_eq!(l.len(), 1);
    assert_eq!(l.head, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    l.push_head(444);
    assert_eq!(l.len(), 2);
    assert_eq!(unsafe { (*l.head.unwrap()).val }, 444);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.head.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.head.unwrap()).next }, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, l.head);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    l.push_head(555);
    assert_eq!(l.len(), 3);
    assert_eq!(unsafe { (*l.head.unwrap()).val }, 555);
    assert_eq!(unsafe { (*(*l.head.unwrap()).next.unwrap()).val }, 444);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.head.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.head.unwrap()).next }, unsafe {
        (*l.tail.unwrap()).prev
    });
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);
    assert_eq!(unsafe { (*(*l.head.unwrap()).next.unwrap()).prev }, l.head);
    assert_eq!(unsafe { (*(*l.head.unwrap()).next.unwrap()).next }, l.tail);
}

#[test]
fn pop_tail() {
    let mut l = LinkedList::new();
    l.push_tail(333);
    l.push_tail(444);
    l.push_tail(555);

    assert_eq!(l.pop_tail(), Some(555));
    assert_eq!(l.len(), 2);
    assert_eq!(unsafe { (*l.head.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 444);
    assert_eq!(unsafe { (*l.head.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.head.unwrap()).next }, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, l.head);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    assert_eq!(l.pop_tail(), Some(444));
    assert_eq!(l.len(), 1);
    assert_eq!(l.head, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 333);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    assert_eq!(l.pop_tail(), Some(333));
    assert_eq!(
        l,
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    );

    assert_eq!(l.pop_tail(), None);
    assert_eq!(
        l,
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    );
}

#[test]
fn pop_head() {
    let mut l = LinkedList::new();
    l.push_tail(333);
    l.push_tail(444);
    l.push_tail(555);

    assert_eq!(l.pop_head(), Some(333));
    assert_eq!(l.len(), 2);
    assert_eq!(unsafe { (*l.head.unwrap()).val }, 444);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 555);
    assert_eq!(unsafe { (*l.head.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.head.unwrap()).next }, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, l.head);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    assert_eq!(l.pop_head(), Some(444));
    assert_eq!(l.len(), 1);
    assert_eq!(l.head, l.tail);
    assert_eq!(unsafe { (*l.tail.unwrap()).val }, 555);
    assert_eq!(unsafe { (*l.tail.unwrap()).prev }, None);
    assert_eq!(unsafe { (*l.tail.unwrap()).next }, None);

    assert_eq!(l.pop_head(), Some(555));
    assert_eq!(
        l,
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    );

    assert_eq!(l.pop_head(), None);
    assert_eq!(
        l,
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    );
}
