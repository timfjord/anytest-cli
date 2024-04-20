use common::Project;

mod common;

#[test]
fn cargotest_line_on_lib() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/lib.rs", 5),
        "cargo test tests::first_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/lib.rs", 13),
        "cargo test tests::third_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/lib.rs", 7),
        "cargo test tests::second_test -- --exact"
    );
}

#[test]
fn cargotest_line_on_modules_with_mod_as_test() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/somemod_test.rs", 5),
        "cargo test somemod_test::test::first_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/somemod_test.rs", 13),
        "cargo test somemod_test::test::third_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/somemod_test.rs", 7),
        "cargo test somemod_test::test::second_test -- --exact"
    );
}

#[test]
fn cargotest_line_without_mod() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/nomod.rs", 2),
        "cargo test nomod::first_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/nomod.rs", 10),
        "cargo test nomod::third_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/nomod.rs", 6),
        "cargo test nomod::second_test -- --exact"
    );
}

#[test]
fn cargotest_line_on_modules() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/somemod.rs", 5),
        "cargo test somemod::tests::first_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/somemod.rs", 13),
        "cargo test somemod::tests::third_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/somemod.rs", 7),
        "cargo test somemod::tests::second_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/nested/mod.rs", 5),
        "cargo test nested::tests::first_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/nested/mod.rs", 13),
        "cargo test nested::tests::third_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/nested/mod.rs", 7),
        "cargo test nested::tests::second_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/too/nested.rs", 5),
        "cargo test too::nested::tests::first_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/too/nested.rs", 13),
        "cargo test too::nested::tests::third_test -- --exact"
    );

    assert_eq!(
        project.test_line("src/too/nested.rs", 7),
        "cargo test too::nested::tests::second_test -- --exact"
    );
}

#[test]
fn cargotest_line_async_tokio() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/lib.rs", 15),
        "cargo test tests::tokio_async_test -- --exact"
    );
}

#[test]
fn cargotest_line_rstest() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/lib.rs", 22),
        "cargo test tests::rstest_test -- --exact"
    );
}

#[test]
fn cargotest_line_async_actix_rt() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("src/lib.rs", 26),
        "cargo test tests::test_actix_rt -- --exact"
    );
}

#[test]
fn cargotest_line_integration_test() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_line("tests/integration_test.rs", 3),
        "cargo test --test integration_test it_adds_two -- --exact"
    );
}

#[test]
fn cargotest_file() {
    let project = Project::new("cargotest/crate");

    assert_eq!(project.test_file("src/lib.rs"), "cargo test");

    assert_eq!(project.test_file("src/main.rs"), "cargo test");

    assert_eq!(project.test_file("src/somemod.rs"), "cargo test somemod::");

    assert_eq!(
        project.test_file("src/nested/mod.rs"),
        "cargo test nested::"
    );

    assert_eq!(
        project.test_file("src/too/nested.rs"),
        "cargo test too::nested::"
    );
}

#[test]
fn cargotest_file_integration_test() {
    let project = Project::new("cargotest/crate");

    assert_eq!(
        project.test_file("tests/integration_test.rs"),
        "cargo test --test integration_test"
    );
}

#[test]
fn cargotest_suite() {
    let project = Project::new("cargotest/crate");

    assert_eq!(project.test_suite("src/lib.rs"), "cargo test");

    assert_eq!(project.test_suite("src/somemod.rs"), "cargo test");

    assert_eq!(project.test_suite("src/nested/mod.rs"), "cargo test");

    assert_eq!(project.test_suite("src/too/nested.rs"), "cargo test");
}

#[test]
fn cargotest_workspace_file() {
    let project = Project::new("cargotest");

    assert_eq!(
        project.test_file("crate/src/lib.rs"),
        "cargo test --package crate"
    );
}
