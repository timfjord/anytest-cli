use common::Project;

mod common;

#[test]
fn test_rspec_line1() {
    let project = Project::new("rspec");

    assert_eq!(
        project.test_line("normal_spec.rb", 1),
        "rspec normal_spec.rb:1"
    );
}

#[test]
fn test_rspec_line2() {
    let project = Project::new("rspec");

    assert_eq!(
        project.test_line("context_spec.rb", 1),
        "rspec context_spec.rb:1"
    );

    assert_eq!(
        project.test_line("context_spec.rb", 2),
        "rspec context_spec.rb:2"
    );

    assert_eq!(
        project.test_line("context_spec.rb", 3),
        "rspec context_spec.rb:3"
    );
}

#[test]
fn test_rspec_file() {
    let project = Project::new("rspec");

    assert_eq!(project.test_file("normal_spec.rb"), "rspec normal_spec.rb");
}

#[test]
fn test_rspec_file_starts_with_test() {
    let project = Project::new("rspec");

    assert_eq!(project.test_file("test_spec.rb"), "rspec test_spec.rb");
}

#[test]
fn test_rspec_suite() {
    let project = Project::new("rspec");

    assert_eq!(project.test_suite("normal_spec.rb"), "rspec ");
}

#[test]
fn test_rspec_turnip() {
    let project = Project::new("rspec");

    assert_eq!(
        project.test_file("spec/math.feature"),
        "rspec spec/math.feature"
    );
}
