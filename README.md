# store
a pointer can refer to its parent

## Example
```rust
fn foo() {
    let mut t = (1, (2, (3, vec![3,2,1])));
    let mut v = store_mut!{ t.1, self.1.1 }; // create a mutable pointer to t.1.1.1
    //                       ^      ^ the path to vec![3,2,1]
    //                       | t.1 as the parent
    println!("{:?}; {:?}", *v, v.pos()); // [3, 2, 1]; (2, (3, [3, 2, 1]))
    //                      ^    ^ &t.1
    //                      | &mut t.1.1.1
    v[1]=4;
    v.pos_mut().0 = 5;
    println!("{:?}; {:?}", *v, v.pos()); // [3, 4, 1]; (5, (3, [3, 4, 1]))
}
```

or 
```rust
#[derive(Debug)]
struct Item(u64);

#[derive(Debug)]
struct Container {
    value: u64,
    item: Item,
}


impl Item {
    // method on &Item using data on Container
    fn sum(self: &Store<Container, Item>) -> u64 {
        self.pos().value + self.0
        //   ^ refer to Container
    }
}

fn foo() {
    let ctn = Container {
        value: 1,
        item: Item(2)
    };

    assert!(store_ref!{ ctn, self.item }.sum() == 3);
}


```

## principle
treat `(a->b, a)` as the pointer to b. it duals to State Monad.

the type `a->b` give a proof that b is a chile of a.