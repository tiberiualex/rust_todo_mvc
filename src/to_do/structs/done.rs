use super::base::Base;
use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::edit::Edit;

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

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
