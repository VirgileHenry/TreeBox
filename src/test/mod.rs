use crate::TreeBox;


#[test]
fn create() {
    let _tree_box: TreeBox<String> = String::from("Hello").into();
    // drop tree_box
}

#[test]
fn childs() {
    let mut tree_box: TreeBox<String> = String::from("Hello").into();
    assert_eq!(tree_box.get(|v| v.len()), 5);
    let mut child = tree_box.create_child(String::from("World"));
    child.mutate_parent(|parent| assert_eq!(parent.get(|v| v.clone()), "Hello"));
    tree_box.mutate_children(|child| assert_eq!(child.get(|v| v.clone()), "World"));
}

