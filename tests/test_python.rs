use common::Project;

mod common;

#[test]
fn test_pytest_line() {
    let project = Project::new("pytest");

    assert_eq!(
        project.test_line("test_class.py", 1),
        "python -m pytest test_class.py::TestClass"
    );

    assert_eq!(
        project.test_line("test_class.py", 2),
        "python -m pytest test_class.py::TestClass::TestNestedClass"
    );

    assert_eq!(
        project.test_line("test_class.py", 3),
        "python -m pytest test_class.py::TestClass::TestNestedClass::test_nestedclass_method"
    );

    assert_eq!(
        project.test_line("test_class.py", 6),
        "python -m pytest test_class.py::TestClass::test_method"
    );

    assert_eq!(
        project.test_line("test_class.py", 10),
        "python -m pytest test_class.py::test_function"
    );
}

#[test]
fn test_pytest_xunit_line() {
    let project = Project::new("nose");

    assert_eq!(
        project.test_line("test_class.py", 1),
        "python -m pytest test_class.py::TestNumbers"
    );

    assert_eq!(
        project.test_line("test_class.py", 2),
        "python -m pytest test_class.py::TestNumbers::test_numbers"
    );

    assert_eq!(
        project.test_line("test_class.py", 6),
        "python -m pytest test_class.py::TestSubclass::test_subclass"
    );

    assert_eq!(
        project.test_line("test_class.py", 9),
        "python -m pytest test_class.py::Test_underscores_and_123"
    );

    assert_eq!(
        project.test_line("test_class.py", 10),
        "python -m pytest test_class.py::Test_underscores_and_123::test_underscores"
    );

    assert_eq!(
        project.test_line("test_class.py", 13),
        "python -m pytest test_class.py::UnittestClass"
    );

    assert_eq!(
        project.test_line("test_class.py", 18),
        "python -m pytest test_class.py::SomeTest::test_foo"
    );

    assert_eq!(
        project.test_line("test_method.py", 3),
        "python -m pytest test_method.py::test_numbers"
    );
}

#[test]
fn test_pytest_xunit_line_no_nearest() {
    let project = Project::new("nose");

    assert_eq!(
        project.test_line("test_method.py", 1),
        "python -m pytest test_method.py"
    );
}

#[test]
fn test_pytest_xunit_file() {
    let project = Project::new("nose");

    assert_eq!(
        project.test_file("test_class.py"),
        "python -m pytest test_class.py"
    );
}

#[test]
fn test_pytest_xunit_suite() {
    let project = Project::new("nose");

    assert_eq!(project.test_suite("test_class.py"), "python -m pytest");
}

#[test]
fn test_pytest_pipenv_line() {
    let project = Project::new("pipenv");

    assert_eq!(
        project.test_line("test_class.py", 1),
        "pipenv run python -m pytest test_class.py::TestNumbers"
    );
}

#[test]
fn test_pytest_pipenv_file() {
    let project = Project::new("pipenv");

    assert_eq!(
        project.test_file("test_class.py"),
        "pipenv run python -m pytest test_class.py"
    );
}

#[test]
fn test_pytest_pipenv_suite() {
    let project = Project::new("pipenv");

    assert_eq!(
        project.test_suite("test_class.py"),
        "pipenv run python -m pytest"
    );
}

#[test]
fn test_pytest_poetry_line() {
    let project = Project::new("poetry");

    assert_eq!(
        project.test_line("test_class.py", 1),
        "poetry run python -m pytest test_class.py::TestNumbers"
    );
}

#[test]
fn test_pytest_poetry_file() {
    let project = Project::new("poetry");

    assert_eq!(
        project.test_file("test_class.py"),
        "poetry run python -m pytest test_class.py"
    );
}

#[test]
fn test_pytest_poetry_suite() {
    let project = Project::new("poetry");

    assert_eq!(
        project.test_suite("test_class.py"),
        "poetry run python -m pytest"
    );
}

#[test]
fn test_pytest_pdm_line() {
    let project = Project::new("pdm");

    assert_eq!(
        project.test_line("test_class.py", 1),
        "pdm run python -m pytest test_class.py::TestNumbers"
    );
}

#[test]
fn test_pytest_pdm_file() {
    let project = Project::new("pdm");

    assert_eq!(
        project.test_file("test_class.py"),
        "pdm run python -m pytest test_class.py"
    );
}

#[test]
fn test_pytest_pdm_suite() {
    let project = Project::new("pdm");

    assert_eq!(
        project.test_suite("test_class.py"),
        "pdm run python -m pytest"
    );
}
