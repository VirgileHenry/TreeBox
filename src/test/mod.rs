use crate::TreeBox;




#[test]
fn create() {
    let _tree_box: TreeBox<String> = String::from("Hello").into();
    // drop tree_box
}

#[test]
fn childs() {
    let mut tree_box: TreeBox<String> = String::from("Hello").into();
    let child = tree_box.create_child(String::from("World")).unwrap();
    {
        let other_child = tree_box.create_child(String::from("Other")).unwrap();
        let mut iter = tree_box.children().unwrap();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
        // drop other_child
        println!("drop other_child of value : {}", other_child.value().unwrap());
    }
    let mut iter = tree_box.children().unwrap();
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
    // drop tree_box and child
    println!("drop tree_box of value : {}", tree_box.value().unwrap());
    println!("drop child of value : {}", child.value().unwrap());
}

#[test]
fn refs() {
    let mut tree_box: TreeBox<String> = String::from("Hello").into();
    let child = tree_box.create_child(String::from("World"));

    let child_ref = tree_box.children().unwrap().next().unwrap();
    drop(child);

    // the previously created child is still alive
    assert_eq!(*child_ref.value().unwrap(), "World".to_string());
    drop(child_ref);
    // but the child is not a child of the tree_box anymore
    assert!(tree_box.children().unwrap().next().is_none());
}