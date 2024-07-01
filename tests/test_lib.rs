use github_actions::greet;

// test the greet function
#[test]
fn test_greet() {
    assert_eq!(greet("world"), "Hello, world!");
}

// test with an empty string
#[test]
fn test_greet_with_empty_string() {
    assert_eq!(greet(""), "Hello, !");
}

// test with name "Ignacio"
#[test]
fn test_greet_with_name_ignacio() {
    assert_eq!(greet("Ignacio"), "Hello, Ignacio!");
}
