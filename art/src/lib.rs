//! # Art
//!
//! 一个用来建模艺术概念的代码库

// 使用 pub use 语句将需要公开的条目重新导出到顶层结构中
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  /// RYB颜色模型的三原色
  #[derive(Debug)]
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
  }
  /// RYB模型的调和色
  #[derive(Debug)]
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;
  /// 将两种等量的原色混合生成调和色
  /// # Examples
  ///
  /// ```
  /// let answer = art::mix(art::PrimaryColor::Red, art::PrimaryColor::Yellow);
  ///
  /// match answer {
  ///   art::SecondaryColor::Green => assert!(true),
  ///   art::SecondaryColor::Orange => assert!(false),
  ///   art::SecondaryColor::Purple => assert!(false),
  /// }
  /// ```
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    // ...
    println!("{:?}", c1);
    println!("{:?}", c2);
    SecondaryColor::Green
  }
}
