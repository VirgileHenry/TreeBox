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
    child.mutate_parent(|parent| assert_eq!(parent, "Hello"));
    tree_box.mutate_children(|child| assert_eq!(child, "World"));
}

#[test]
fn rec() {
    let mut parent: TreeBox<String> = String::from("Hello").into();
    let mut child1 = parent.create_child(String::from("Child 1"));
    let mut grandchild1 = child1.create_child(String::from("Child 1 -> Child 1"));
    let _grandgrandchild1 = grandchild1.create_child(String::from("Child 1 -> Child 1 -> Child 1"));
    let _child2 = parent.create_child(String::from("Child 2"));
    let _grandchild2 = child1.create_child(String::from("Child 1 -> Child 2"));

    parent.mutate_children_rec(|s| {
        println!("{}", s);
    });
}

#[test]
fn parent_get_rec() {
    let mut parent: TreeBox<Option<String>> = Some(String::from("Hello")).into();
    let mut child1 = parent.create_child(None);
    let grandchild1 = child1.create_child(None);
    let result = grandchild1.get_parent_rec(
        |s| {
            println!("try get");
            s.clone().map(|s| s)
        },
        |_, p| {
            println!("try get failed: parent fallback");
            p.or(Some("No Value at all !".to_string())).unwrap()
        });
    assert_eq!(result, "Hello".to_string());
}