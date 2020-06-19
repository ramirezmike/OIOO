# OIOO

A data structure built in Rust intended as an alternative to FIFO or LIFO: One-in, One-out. 

## Features
  * stores items pushed into the data structure which can later be retrieved randomly
  * each item is padded with a number of empty slots based on recommended social-distance guidelines 
  * max capacity of the OIOO is determined upon creation
  * excess items added to the OIOO are contained in a queue which is automatically used to fill the main store when space becomes available.
  * support for multiple Phases which alter the capabilities of the OIOO


```rust
// create a Phase Two (50%) capacity OIOO 
let mut oioo = oioo::OIOO::<usize>::new(oioo::Phase::Two { occupancy: 10 }); 
oioo.one_in(10); 
oioo.one_in(20);
oioo.one_in(30);
oioo.one_in(40);
oioo.one_in(50);
oioo.one_in(60); // exceeds occupancy, contained in queue
 
// random from 10, 20, 30, 40 or 50. The value "60" is automatically pushed into the empty space.
println!("{}", oioo.one_out().unwrap() as usize); 

// random from 10, 20, 30, 40, 50 or 60, excluding value printed above
println!("{}", oioo.one_out().unwrap() as usize); 
```
