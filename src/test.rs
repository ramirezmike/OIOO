use super::*;

fn get_number_in_store<T>(store: &Vec::<Option<T>>) -> usize {
    store.iter()
         .filter(|x| x.is_some())
         .collect::<Vec<_>>()
         .len()
}

#[test]
fn test() {
    let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: 10 }); 
    oioo.one_in(10); 
    oioo.one_in(20);
    oioo.one_in(30);
    oioo.one_in(40);
    oioo.one_in(50);
    oioo.one_in(60); // exceeds occupancy, contained in queue
    
    // random from 10, 20, 30, 40 or 50
    println!("{}", oioo.one_out().unwrap() as usize); 
    // random from 10, 20, 30, 40, 50 or 60, excluding value printed above
    println!("{}", oioo.one_out().unwrap() as usize); 
}

#[test]
fn test_one_in() {
    let mut oioo = OIOO::<usize>::new(Phase::One { occupancy: 4, is_essential: true });
    assert!(oioo.store.len() == 0);
    oioo.one_in(3);
    assert_eq!(oioo.store.len(), SOCIAL_DISTANCE + 1);
}

#[test]
fn test_one_in_other_type() {
    let mut oioo = OIOO::<&str>::new(Phase::One { occupancy: 4, is_essential: true });
    assert!(oioo.store.len() == 0);
    oioo.one_in(&"test");
    assert_eq!(oioo.store.len(), SOCIAL_DISTANCE + 1);
}

#[test]
fn test_one_in_is_essential() {
    let mut oioo = OIOO::<usize>::new(Phase::One { occupancy: 4, is_essential: true });
    assert!(oioo.store.len() == 0);
    oioo.one_in(3);
    assert_eq!(get_number_in_store(&oioo.store), 1);
}

#[test]
fn test_one_in_is_not_essential() {
    let mut oioo = OIOO::<usize>::new(Phase::One { occupancy: 4, is_essential: false });
    assert!(oioo.store.len() == 0);
    oioo.one_in(3);
    assert_eq!(get_number_in_store(&oioo.store), 0);
}

#[test]
fn test_one_in_max_capacity_is_less_phase_one() {
    let occupancy = 8;
    let mut oioo = OIOO::<usize>::new(Phase::One { occupancy: occupancy, is_essential: true });
    assert!(oioo.store.len() == 0);

    for i in 0..occupancy {
        oioo.one_in(i);
    }

    assert_eq!(get_number_in_store(&oioo.store), occupancy / 4);
}

#[test]
fn test_one_in_max_capacity_is_less_phase_two() {
    let occupancy = 8;
    let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: occupancy });
    assert!(oioo.store.len() == 0);

    for i in 0..occupancy {
        oioo.one_in(i);
    }

    assert_eq!(get_number_in_store(&oioo.store), occupancy / 2);
}

#[test]
fn test_one_in_store_in_queue() {
    let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
    let count:usize = 10;
    assert!(oioo.store.len() == 0);
    for x in 0..count {
        oioo.one_in(x);
    }
    assert_eq!(oioo.store.len(), (SOCIAL_DISTANCE + 1) * count);
    assert_eq!(oioo.queue.len(), 0);

    oioo.one_in(count + 1);
    assert_eq!(oioo.store.len(), (SOCIAL_DISTANCE + 1) * count);
    assert_eq!(oioo.queue.len(), 1);
}

#[test]
fn test_one_out() {
    let mut oioo = OIOO::<usize>::new(Phase::Two { occupancy: 20 });
    let value = 3;
    assert!(oioo.store.len() == 0);
    oioo.one_in(value);
    assert_eq!(oioo.store.len(), SOCIAL_DISTANCE + 1);

    let first_result = oioo.one_out().unwrap();
    assert_eq!(first_result, value);
    assert!(oioo.store.len() == 0);

    let second_result = oioo.one_out();
    assert_eq!(second_result, None);
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

