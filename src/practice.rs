pub mod _struct;
pub mod enumerate;
pub mod general_concept;
pub mod ownership;

pub fn go() {
  general_concept::run();
  ownership::run();
  _struct::run();
  enumerate::run();
}
