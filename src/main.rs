use dmap::diff;
use dmap::diff::Diff::{Remove, Add, Changed};
use std::collections::HashSet;
use dmap::diff::{Diff, DiffReport};
use std::iter::FromIterator;
use dmap::common::InputType;

fn create_paths(test_no: i32) -> (String, String) {
    (format!("resources/test_{}/dir_1", test_no), format!("resources/test_{}/dir_2", test_no))
}

fn test_1() {
    
    
    
    let (dir1, dir2) = create_paths(1);
    
    let expected = DiffReport::create(vec![ Remove(String::from("hello_world.txt")) ]);
    
    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));
        
    assert_eq!(expected, actual);
}

fn test_2() {
    let (dir1, dir2) = create_paths(2);

    let expected = DiffReport::create(vec![ Add(String::from("hello_world.txt")) ]);

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_3() {
    let (dir1, dir2) = create_paths(3);

    let expected = DiffReport::create(vec![ Changed(String::from("hello_world.txt")) ]);

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_4() {
    let (dir1, dir2) = create_paths(4);

    let expected = DiffReport::create(vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("foo.txt")) ]);

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_5() {
    let (dir1, dir2) = create_paths(5);

    let expected = DiffReport::create(vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("hello_world.txt")) ]);

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));
    
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

fn test_10() {
    let (dir1, dir2) = create_paths(10);

    let expected = DiffReport::create(vec![ Remove(String::from("child/foo.txt")) ]);

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}


fn test_11() {
    let (dir1, dir2) = create_paths(11);

    let expected = DiffReport::create(vec![ Add(String::from("child/foo.txt")) ]);

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}







fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_10();
    test_11();
    
    println!("{}", '\u{2705}')
}
