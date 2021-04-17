use super::base::Base;

pub struct Done {
  pub super_struct: Base
}

impl Done {
  pub fn new(input_title: &str) -> Done {
    let input_status: String = String::from("done");
    let base: Base = Base::new(input_title, &input_status);
    return Done{super_struct: base}
  }
}
