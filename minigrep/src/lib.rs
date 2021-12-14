use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  // &'static str代表字符串字面量类型,这里是指Err中的'not enough arguments'
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    // env::var函数会返回一个Result作为结果
    // 只有在环境变量被设置时，该结果才会是包含环境变量值的Ok变体
    // 而在环境变量未被设置时，该结果则会是一个Err变体
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

// Box<dyn Error> => trait对象
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? 运算符取代了expect
  // ? 运算符可以将错误值返回给函数的调 用者来进行处理。
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }

  Ok(())
}
// 显式生命周期'a
// 生命周期参数指定了哪一个参数的生命周期会和返回值的生命周期产生关联。
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line)
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}
