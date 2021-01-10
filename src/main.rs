use dmap::diff;
use dmap::diff::Diff::{Remove, Add, Changed};
use dmap::diff::{DiffReport};
use dmap::common::InputType;

fn create_paths(test_no: i32) -> (String, String) {
    (format!("resources/test_{}/dir_1", test_no), format!("resources/test_{}/dir_2", test_no))
}

fn test_1() {

    let (dir1, dir2) = create_paths(1);
    
    let expected = Ok(DiffReport::create(vec![ Remove(String::from("hello_world.txt")) ]));
    
    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));
        
    assert_eq!(expected, actual);
}

fn test_2() {
    let (dir1, dir2) = create_paths(2);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("hello_world.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_3() {
    let (dir1, dir2) = create_paths(3);

    let expected = Ok(DiffReport::create(vec![ Changed(String::from("hello_world.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_4() {
    let (dir1, dir2) = create_paths(4);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("foo.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_5() {
    let (dir1, dir2) = create_paths(5);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("hello_world.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));
    
    assert_eq!(expected, actual);
}

fn test_6() {
    let (dir1, dir2) = create_paths(6);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("baz.txt")), Remove(String::from("bar.txt")), Remove(String::from("hello_world.txt")), Changed(String::from("foo.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_7() {
    let (dir1, dir2) = create_paths(7);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("bar.txt")), Add(String::from("foo.txt")), Remove(String::from("baz.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_8() {
    let (dir1, dir2) = create_paths(8);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("bar.txt")), Add(String::from("hello_world.txt")), Remove(String::from("baz.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_9() {
    let (dir1, dir2) = create_paths(9);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("bar.txt")), Add(String::from("hello_world.txt")), Remove(String::from("baz.txt")), Changed(String::from("foo.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_10() {
    let (dir1, dir2) = create_paths(10);

    let expected = Ok(DiffReport::create(vec![ Remove(String::from("child/foo.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}


fn test_11() {
    let (dir1, dir2) = create_paths(11);

    let expected = Ok(DiffReport::create(vec![ Add(String::from("child/foo.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn test_12() {
    let (dir1, dir2) = create_paths(12);

    let expected = Ok(DiffReport::create(vec![ Changed(String::from("child/foo.txt")) ]));

    let actual = diff(InputType::Directory(dir1.as_ref()), InputType::Directory(dir2.as_ref()));

    assert_eq!(expected, actual);
}

fn run_basic_tests() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_6();
    test_7();
    test_8();
    test_9();
    test_10();
    test_11();
    test_12();
}

fn main() {
    run_basic_tests();
    
    println!("{}", '\u{2705}')
}
