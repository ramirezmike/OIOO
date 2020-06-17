#![allow(dead_code)]

use rand::{ Rng };

struct OIOO<T> {
    store: Vec::<Option<T>>,
    queue: Vec::<T>,
    social_distance: usize,
    capacity: usize
}

impl<T> OIOO<T> {
    pub fn new() -> OIOO<T> {
        OIOO {
            store: Vec::<Option<T>>::new(),
            queue: Vec::<T>::new(),
            social_distance: 6, 
            capacity: 10
        }
    }

    pub fn one_in(self: &mut Self, item: T) {
        if !self.at_capacity() {
            self.store.push(Some(item));
            self.add_social_distance();
        } else {
            self.queue.push(item);
        }
    }

    pub fn one_out(self: &mut Self) -> Option<T> {
        if self.store.len() == 0 { return None; }

        let mut rng = rand::thread_rng();
        let out_index = rng.gen_range(0, self.store.iter()
                                                   .filter(|x| x.is_some())
                                                   .collect::<Vec<_>>()
                                                   .len()) * (self.social_distance + 1);

        let out = match self.store[out_index].is_some() {
                    true => {
                        let social_distance_index = out_index + self.social_distance + 1;
                        let mut out_and_social_distance = self.store.drain(out_index..social_distance_index)
                                                                    .collect::<Vec<_>>();
                        Some(out_and_social_distance.remove(0).unwrap())
                    }
                    false => None
                  };

        if !self.queue.is_empty() {
            let first_in_queue = self.queue.remove(0);
            self.one_in(first_in_queue);
        }

        out
    }

    fn at_capacity(self: &Self) -> bool {
        (self.store.len() / (self.social_distance + 1)) >= self.capacity
    }

    fn add_social_distance(self: &mut Self) {
        for _ in 0..self.social_distance {
            self.store.push(None);
        }
    }

    fn remove_social_distance(self: &mut Self) {
        for _ in 0..self.social_distance {
            self.store.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_in() {
        let mut oioo = OIOO::<usize>::new();
        assert!(oioo.store.len() == 0);
        oioo.one_in(3);
        assert_eq!(oioo.store.len(), oioo.social_distance + 1);
    }

    #[test]
    fn test_one_in_store_in_queue() {
        let mut oioo = OIOO::<usize>::new();
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
        let mut oioo = OIOO::<usize>::new();
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
        let mut oioo = OIOO::<usize>::new();
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
        let mut oioo_1 = OIOO::<usize>::new();
        let mut oioo_2 = OIOO::<usize>::new();
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
