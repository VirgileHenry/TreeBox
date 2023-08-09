## TreeBox

Tree box is a box holding a value in the heap, with an additional tree structure. It has an optionnal parent, and a list of children.

It was primarly designed for the transform component of the propellant game engine, allowing a tree-like hierarchy in the transforms while still storing them in a component table for fast iterations.

## Features

This struct can :
- Create a new tree box from a value.
- Set the tree box parent.
- Get a ref to the parent.
- A tree box can iterate over its children.

## Usage

```rust
// create a new tree box, with no parents nor child.
// TreeBox<T> implements From<T> for easy creation.
let mut tree_box: TreeBox<String> = String::from("Hello").into();
```
    
```rust
// Creates a child from the parent.
// This will also create a TreeBox, but will set his parent to the provided tree box,
// and the parent will get a reference to his child.
let mut child = tree_box.create_child(String::from("World"));
```

```rust
// we can use the set_parent method to change the parent of a tree box.
child.set_parent(None);
child.set_parent(Some(&mut tree_box));
```

```rust
// the parent can also iterate over it's children.
// parents are storing weak references to their child, but when accessing them,
// they are upgraded to shared references. Here, we are getting ownership of the child
// when iterating over, but these ownerships are dropped after use and the child tree box remains the sole owner.
for child in tree_box.children() {
    println!("{}", child);
}
```

## Todo

There are still quite a few things to do, as this wa created over a few hours when it was needed.
- propper testing
- benchmarking + optimization
- fixing bugs (I'm sure there are some)