# tododo

## backend

- multithreading
- implement parallel iteration using rayon
- animations through texture swapping
- putting enemy data into json for modularity and quicker changing of values

## goody stuff

- organize and structure project properly

```txt
src/
    player/..
    physics/..
    game/
        scenemanager.rs 
        levelmanager.rs 
        storage.rs etc.
    main.rs
Cargo.toml
..
```

## libraries to look into

```toml
[dependencies]
rayon = "*"
```

## example notes

```rust
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

![eh](https://i.pinimg.com/564x/63/ec/67/63ec67b961aef9a44ec6fa5438328614.jpg)
