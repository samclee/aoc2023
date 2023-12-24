use std::collections::HashSet;

use super::{is_symbol, get_adjacent_coords, part1};
use crate::my_vec::MyVec;

#[test]
fn test_myvec() {
    let c1 = MyVec {
        x: 10,
        y: 10
    };
    let c2 = MyVec {
        x: 10,
        y: 10
    };

    assert_eq!(c1, c2);

    let mut my_set = HashSet::new();
    my_set.insert(c1);
    my_set.insert(c2);
    assert_eq!(1, my_set.len());

    let c3 = c1 + c2;
    assert_eq!( MyVec { x: 20, y: 20 }, c3 );

    let c4 = c3 - MyVec{ x:15, y:17 };
    assert_eq!( MyVec { x: 5, y: 3 }, c4);

    let c5 = MyVec::new(3, 4);
    assert_eq!( MyVec { x: 3, y: 4}, c5 );

    let coords = get_adjacent_coords(10, 10);
    assert_eq!(8, coords.len());
    assert_eq!(MyVec::new(11, 10), *coords.get(0).unwrap());
    assert_eq!(MyVec::new(11, 11), *coords.get(1).unwrap());
    assert_eq!(MyVec::new(10, 11), *coords.get(2).unwrap());
    assert_eq!(MyVec::new(9, 11), *coords.get(3).unwrap());
    assert_eq!(MyVec::new(9, 10), *coords.get(4).unwrap());
    assert_eq!(MyVec::new(9, 9), *coords.get(5).unwrap());
    assert_eq!(MyVec::new(10, 9), *coords.get(6).unwrap());
    assert_eq!(MyVec::new(11, 9), *coords.get(7).unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(156, part1("156.txt"));
    assert_eq!(413, part1("413.txt"));
    assert_eq!(925, part1("925.txt"));
    assert_eq!(4361, part1("4361.txt"));
    assert_eq!(540131, part1("input.txt"));
}

#[test]
fn test_is_symbol() {
    assert!(is_symbol('$'));
    assert!(!is_symbol('9'));
    assert!(!is_symbol('.'));
}

