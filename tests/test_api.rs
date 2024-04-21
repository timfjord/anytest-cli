use anytest::{Context, Scope};

#[test]
fn test_build_command() {
    let context = Context::new(
        Some("tests/fixtures/cargotest/crate"),
        "tests/integration_test.rs",
        Some(3),
        Some(Scope::Line),
    )
    .unwrap();
    let command = anytest::build_command(&context).unwrap();

    assert_eq!(command.get_program(), "cargo");
    assert_eq!(
        command
            .get_args()
            .map(|arg| arg.to_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "test",
            "--test",
            "integration_test",
            "it_adds_two",
            "--",
            "--exact"
        ]
    );
}
