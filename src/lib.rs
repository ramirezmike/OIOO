use rand::{ Rng };

/// Dictates the current Phase, which limits the capabilities of an OIOO instance.
pub enum Phase {
    One { 
        occupancy: usize, 
        is_essential: bool 
    },
    Two { occupancy: usize },
}

/// A data structure intended as an alternative to FIFO or LIFO: One-in, One-out. Items are 
/// pushed into the data structure and are retrieved randomly. Each item is padded with
/// a number of empty slots based on recommended social-distance guidelines. The capacity
/// of the OIOO is set upon creation; any excess items are contained in a queue which is 
/// automatically used to fill the main store when space becomes available. 
pub struct OIOO<T> {
    /// Used as primary storage of items pushed into the OIOO up until the capacity is hit.
    store: Vec::<Option<T>>,
    /// Used as overflow of items that can't fit in in store due to capacity limitations.
    queue: Vec::<T>,
    /// Number of empty spaces between items.
    social_distance: usize,
    /// Total number of items contained in "store" determined by Phase used to initialize the OIOO.
    capacity: usize
}

impl<T> OIOO<T> {
    /// Creates a new instance of an OIOO based on the selected Phase.
    ///
    /// <b>Using Phase One</b>
    /// <ul>
    ///     <li>capacity is set to 25% of the passed in Phase::One's occupancy value</li>
    ///     <li>if <b>is_essential</b> is false, the OIOO will be unable to contain items in the main store</li>
    /// </ul>
    ///
    /// <b>Using Phase Two</b>
    /// <ul>
    ///     <li>capacity is set to 50% of the passed in Phase::Two's occupancy value</li>
    /// </ul>
    pub fn new(phase: Phase) -> OIOO<T> {
        OIOO {
            store: Vec::<Option<T>>::new(),
            queue: Vec::<T>::new(),
            social_distance: 6,
            capacity: match phase {
                // Phase One 25% occupancy for essentials 
                Phase::One { occupancy, is_essential } => {
                    if is_essential { occupancy / 4 } else { 0 }
                },
                // Phase Two 50% occupancy regardless of essentiality
                Phase::Two { occupancy } => occupancy / 2
            }
        }
    }

    /// Pushes an item into the store if there is space. If the store is
    /// at capacity, the item will be contained "outside" in a queue that will
    /// be pulled from once space becomes available. Each item added into
    /// the store will have an appropriate amount of social distance between
    /// it and the next item added to the store.
    ///
    /// # Example
    ///
    /// ```
    /// // create a Phase Two (50%) capacity OIOO 
    /// let mut oioo = oioo::OIOO::<usize>::new(oioo::Phase::Two { occupancy: 2 }); 
    /// oioo.one_in(10); // contained in store
    /// oioo.one_in(20); // exceeds storage, gets contained in outer queue
    /// ```
    pub fn one_in(self: &mut Self, item: T) {
        if !self.at_capacity() {
            self.store.push(Some(item));
            self.add_social_distance();
        } else {
            self.queue.push(item);
        }
    }

    /// Returns a random item from the store if one exists. If the store was
    /// at capacity prior to the call, item will be contained "outside" in a queue that will
    /// be pulled from once space becomes available.
    ///
    /// # Example
    ///
    /// ```
    /// // create a Phase Two (50%) capacity OIOO 
    /// let mut oioo = oioo::OIOO::<usize>::new(oioo::Phase::Two { occupancy: 10 }); 
    /// oioo.one_in(10); 
    /// oioo.one_in(20);
    /// oioo.one_in(30);
    /// oioo.one_in(40);
    /// oioo.one_in(50);
    /// oioo.one_in(60); // exceeds occupancy, contained in queue
    /// 
    /// // random from 10, 20, 30, 40 or 50
    /// println!("{}", oioo.one_out().unwrap() as usize); 
    /// // random from 10, 20, 30, 40, 50 or 60, excluding value printed above
    /// println!("{}", oioo.one_out().unwrap() as usize); 
    /// ```
    pub fn one_out(self: &mut Self) -> Option<T> {
        if self.store.len() == 0 { return None; }

        let mut rng = rand::thread_rng();
        let out_index = rng.gen_range(0, self.store.iter()
                                                   .filter(|x| x.is_some())
                                                   .collect::<Vec<_>>()
                                                   .len()) * (self.social_distance + 1);

        match self.store[out_index].is_some() {
          true => {
              let social_distance_index = out_index + self.social_distance + 1;
              let mut out_and_social_distance = self.store.drain(out_index..social_distance_index)
                                                          .collect::<Vec<_>>();
              if !self.queue.is_empty() {
                  let first_in_queue = self.queue.remove(0);
                  self.one_in(first_in_queue);
              }

              Some(out_and_social_distance.remove(0).unwrap())
          }
          false => None
        }
    }

    fn at_capacity(self: &Self) -> bool {
        (self.store.len() / (self.social_distance + 1)) >= self.capacity
    }

    fn add_social_distance(self: &mut Self) {
        for _ in 0..self.social_distance {
            self.store.push(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_in() {
        let mut oioo = OIOO::<usize>::new(Phase::One { occupancy: 4, is_essential: true });
        assert!(oioo.store.len() == 0);
        oioo.one_in(3);
        assert_eq!(oioo.store.len(), oioo.social_distance + 1);
    }

    #[test]
    fn test_one_in_store_in_queue() {
        let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
        let count:usize = 10;
        assert!(oioo.store.len() == 0);
        for x in 0..count {
            oioo.one_in(x);
        }
        assert_eq!(oioo.store.len(), (oioo.social_distance + 1) * count);
        assert_eq!(oioo.queue.len(), 0);

        oioo.one_in(count + 1);
        assert_eq!(oioo.store.len(), (oioo.social_distance + 1) * count);
        assert_eq!(oioo.queue.len(), 1);
    }

    #[test]
    fn test_one_out() {
        let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
        let value = 3;
        assert!(oioo.store.len() == 0);
        oioo.one_in(value);
        assert_eq!(oioo.store.len(), oioo.social_distance + 1);

        let first_result = oioo.one_out().unwrap();
        assert_eq!(first_result, value);
        assert!(oioo.store.len() == 0);

        let second_result = oioo.one_out();
        assert_eq!(second_result, None);
    }

    fn get_number_in_store<T>(store: &Vec::<Option<T>>) -> usize {
        store.iter()
             .filter(|x| x.is_some())
             .collect::<Vec<_>>()
             .len()
    }

    #[test]
    fn test_one_out_inserts_into_store() {
        let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
        let count:usize = 11;
        for x in 0..count {
            oioo.one_in(x);
        }

        assert_eq!(1, oioo.queue.len());
        assert_eq!(10, get_number_in_store(&oioo.store)); 
                       
        oioo.one_out();
                       
        assert_eq!(0, oioo.queue.len());
        assert_eq!(10, get_number_in_store(&oioo.store)); 
    }

    #[test]
    fn test_one_out_is_random() {
        let mut oioo_1 = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
        let mut oioo_2 = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
        
        let count:usize = 11;

        let mut keep_trying = true;
        while keep_trying {
            for x in 0..count {
                oioo_1.one_in(x);
                oioo_2.one_in(x);
            }

            for _ in 0..count {
                if oioo_1.one_out() != oioo_2.one_out() {
                    keep_trying = false; 
                    break;
                }
            }
        }
    }
}
