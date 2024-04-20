use common::Project;

mod common;

#[test]
fn test_jest_line() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_line("__tests__/normal-test.js", 2),
        "jest --runTestsByPath -t '^Math' -- __tests__/normal-test.js"
    );

    assert_eq!(
        project.test_line("__tests__/normal-test.js", 3),
        "jest --runTestsByPath -t '^Math Addition' -- __tests__/normal-test.js"
    );

    assert_eq!(
        project.test_line("__tests__/normal-test.js", 4),
        "jest --runTestsByPath -t '^Math Addition adds two numbers$' -- __tests__/normal-test.js"
    );
}

#[test]
fn test_jest_line_context() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_line("__tests__/context-test.js", 1),
        "jest --runTestsByPath -t '^Math' -- __tests__/context-test.js"
    );

    assert_eq!(
        project.test_line("__tests__/context-test.js", 2),
        "jest --runTestsByPath -t '^Math Addition' -- __tests__/context-test.js"
    );

    assert_eq!(
        project.test_line("__tests__/context-test.js", 3),
        "jest --runTestsByPath -t '^Math Addition adds two numbers$' -- __tests__/context-test.js"
    );
}

#[test]
fn test_jest_line_coffee() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_line("__tests__/normal-test.coffee", 1),
        "jest --runTestsByPath -t '^Math' -- __tests__/normal-test.coffee"
    );

    assert_eq!(
        project.test_line("__tests__/normal-test.coffee", 2),
        "jest --runTestsByPath -t '^Math Addition' -- __tests__/normal-test.coffee"
    );

    assert_eq!(
        project.test_line("__tests__/normal-test.coffee", 3),
        "jest --runTestsByPath -t '^Math Addition adds two numbers$' -- __tests__/normal-test.coffee"
    );
}

#[test]
fn test_jest_line_react() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_line("__tests__/normal-test.jsx", 1),
        "jest --runTestsByPath -t '^Math' -- __tests__/normal-test.jsx"
    );

    assert_eq!(
        project.test_line("__tests__/normal-test.jsx", 2),
        "jest --runTestsByPath -t '^Math Addition' -- __tests__/normal-test.jsx"
    );

    assert_eq!(
        project.test_line("__tests__/normal-test.jsx", 3),
        "jest --runTestsByPath -t '^Math Addition adds two numbers$' -- __tests__/normal-test.jsx"
    );
}

#[test]
fn test_jest_line_no_nearest() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_line("__tests__/normal-test.js", 1),
        "jest --runTestsByPath -- __tests__/normal-test.js"
    );
}

#[test]
fn test_jest_file() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_file("__tests__/normal-test.js"),
        "jest --runTestsByPath -- __tests__/normal-test.js"
    );
}

#[test]
fn test_jest_file_outside_tests_folder() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_file("outside-test.js"),
        "jest --runTestsByPath -- outside-test.js"
    );
}

#[test]
fn test_jest_suite() {
    let project = Project::new("jest");

    assert_eq!(
        project.test_suite("__tests__/normal-test.js"),
        "jest --runTestsByPath"
    );
}
