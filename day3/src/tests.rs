use super::*;
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
}