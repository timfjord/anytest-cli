use common::Project;

mod common;

#[test]
fn test_espec_line() {
    let project = Project::new("espec");

    assert_eq!(
        project.test_line("normal_spec.exs", 3),
        "mix espec normal_spec.exs:3"
    );
}

#[test]
fn test_espec_file() {
    let project = Project::new("espec");

    assert_eq!(
        project.test_file("normal_spec.exs"),
        "mix espec normal_spec.exs"
    );
}

#[test]
fn test_espec_suite() {
    let project = Project::new("espec");

    assert_eq!(project.test_suite("normal_spec.exs"), "mix espec");
}

#[test]
fn test_exunit_without_mix_line() {
    let project = Project::new("exunit");

    assert_eq!(
        project.test_line("normal_test.exs", 6),
        "elixir normal_test.exs"
    );
}

#[test]
fn test_exunit_without_mix_file() {
    let project = Project::new("exunit");

    assert_eq!(
        project.test_file("normal_test.exs"),
        "elixir normal_test.exs"
    );
}

#[test]
fn test_exunit_without_mix_suite() {
    let project = Project::new("exunit");

    assert_eq!(project.test_suite("normal_test.exs"), "elixir *.exs");
}

#[test]
fn test_exunit_mix_first_line() {
    let project = Project::new("exunit/mix");

    assert_eq!(
        project.test_line("normal_test.exs", 1),
        "mix test normal_test.exs"
    );
}

#[test]
fn test_exunit_mix_specific_line() {
    let project = Project::new("exunit/mix");

    assert_eq!(
        project.test_line("normal_test.exs", 6),
        "mix test normal_test.exs:6"
    );
}

#[test]
fn test_exunit_mix_file() {
    let project = Project::new("exunit/mix");

    assert_eq!(
        project.test_file("normal_test.exs"),
        "mix test normal_test.exs"
    );
}

#[test]
fn test_exunit_mix_suite() {
    let project = Project::new("exunit/mix");

    assert_eq!(project.test_suite("normal_test.exs"), "mix test");
}
