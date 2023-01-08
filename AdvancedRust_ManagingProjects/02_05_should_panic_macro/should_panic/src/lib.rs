pub struct Person {
    name: String
}

impl Person {
    pub fn new(name: String) -> Person {
        // panic if the name is empty
        if name.is_empty() {
            panic!("A Person needs a name!");
        }

        Person{name}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_test() {
        let new_person = Person::new("Steve".to_string());
        assert_eq!(new_person.name, "Steve".to_string());
    }

    // Test pass because function should panic.
    #[test]
    //#[should_panic]
    #[should_panic(expected = "needs a name")]
    fn person_panic_test() {
        let new_person = Person::new("".to_string());
        assert_eq!(new_person.name, "Steve".to_string());
    }
}