## TreeBox

Tree box is a box holding a value in the heap, with an additional tree structure. It has an optionnal parent, and a list of children.

It was primarly designed for the transform component of the propellant game engine, allowing a tree-like hierarchy in the transforms while still storing them in a component table for fast iterations. This imply a very particular use case, with a specific philosophy: you mutate it by passing closures to it. This allows that there are no refs to this structure, and the borrow calls can be done without error handling.

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
// values are accessed by passing a specific closure that returns what you want.
// this is super specific, as you can't get a reference to the inner value.
let length = tree_box.get(|v| v.len());
```

```rust
// finally, you can mutate any of the value, the parent value, or the child values by passing a mutating funciton.
tree_box.mutate(|v| v.push_str(" World!"));
tree_box.mutate_parent(|v| v.push_str("Hello ")); // won't do anything as the parent is None
tree_box.mutate_children(|v| v.push_str("!")); // will mutate the child value
```


## Todo

There are still quite a few things to do, as this wa created over a few hours when it was needed.
- propper testing
- benchmarking + optimization
- fixing bugs (I'm sure there are some)