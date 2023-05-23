use linked_list::LinkedList;

#[derive(Debug)]
struct Test(Vec<u8>);

impl Drop for Test {
    fn drop(&mut self) {
        println!("drop Test");
    }
}

fn main() {
    /*let mut first = Node {
        next: None,
        prev: None,
        val: 0,
    };
    let mut second = Node {
        next: None,
        prev: Some(&mut first as *mut Node),
        val: 1,
    };
    first.next = Some(&mut second as *mut Node);

    println!("{:?}", unsafe { &*first.next.unwrap() });*/

    {
        let mut l = LinkedList::new();
        for i in 0..100 {
            l.push_tail(Test(vec![0u8; 1000_000]));
        }
        dbg!(l);
        //let v = l.pop_tail();
    }
    loop {}

    //dbg!(unsafe { v });
}
