use super::base::Base;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");
        return Pending { super_struct: base }
    }
}

#[cfg(test)]

mod pending_test {
    use super::Pending;
    #[test]
    fn new() {
        let expected_status: String = String::from("pending");
        let title: String = String::from("washing");
        let expected_title: String = String::from("washing");

        let pending: Pending = Pending::new(&title);
        assert_eq!(expected_status, pending.super_struct.status);
        assert_eq!(expected_title, pending.super_struct.title);
    }
}