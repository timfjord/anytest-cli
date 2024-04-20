use common::Project;

mod common;

#[test]
fn test_zigtest_line() {
    let project = Project::new("zigtest");

    assert_eq!(
        project.test_line("normal.zig", 9),
        "zig test normal.zig --test-filter 'numbers 2'"
    );
}

#[test]
fn test_zigtest_file() {
    let project = Project::new("zigtest");

    assert_eq!(project.test_file("normal.zig"), "zig test normal.zig");
}

#[test]
fn test_zigtest_suite() {
    let project = Project::new("zigtest");

    assert_eq!(project.test_suite("normal.zig"), "zig build test");
}
