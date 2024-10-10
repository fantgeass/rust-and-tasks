struct User {
    name: String,
    age: i32,
}

impl User {
    fn label(&self) -> String {
        format!("{}, {}", self.name, self.age)
    }
}

fn label(user: &User) -> String {
    format!("{}, {}", user.name, user.age)
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let user = User {
            name: "John".into(),
            age: 25,
        };
        assert_eq!(user.label(), "John, 25");
        assert_eq!(label(&user), "John, 25");
    }
}
