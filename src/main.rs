use dmap::compare;
use dmap::diff::Diff::{Remove, Add, Changed};
use std::collections::HashSet;
use dmap::diff::Diff;
use std::iter::FromIterator;


fn create_paths(test_no: i32) -> (String, String) {
    (format!("resources/test_{}/dir_1", test_no), format!("resources/test_{}/dir_2", test_no))
}

fn test_1() {
    let (dir1, dir2) = create_paths(1);
    
    let expected = vec![ Remove(String::from("hello_world.txt")) ];
    
    let actual = compare(dir1.as_ref(), dir2.as_ref());
        
    assert_eq!(expected, actual);
}

fn test_2() {
    let (dir1, dir2) = create_paths(2);

    let expected = vec![ Add(String::from("hello_world.txt")) ];

    let actual = compare(dir1.as_ref(), dir2.as_ref());

    assert_eq!(expected, actual);
}

fn test_3() {
    let (dir1, dir2) = create_paths(3);

    let expected = vec![ Changed(String::from("hello_world.txt")) ];

    let actual = compare(dir1.as_ref(), dir2.as_ref());

    assert_eq!(expected, actual);
}

fn test_4() {
    let (dir1, dir2) = create_paths(4);

    let expected = vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("foo.txt")) ];

    let actual = compare(dir1.as_ref(), dir2.as_ref());

    assert_eq!(expected, actual);
}

fn test_5() {
    let (dir1, dir2) = create_paths(5);

    let expected = vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("hello_world.txt")) ];

    let actual = compare(dir1.as_ref(), dir2.as_ref());

    
    
    
    assert_eq!(expected, actual);
}

fn test_6() {

}

fn test_7() {

}

fn test_8() {

}

fn test_9() {

}








fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
}
