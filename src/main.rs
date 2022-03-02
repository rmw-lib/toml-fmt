use std::error::Error;
use std::io::{self, Read, Write};
use std::{env, fs};

fn fmt(txt: &str) -> String {
  let value: toml::value::Value = toml::de::from_str(txt).unwrap();
  toml::ser::to_string_pretty(&value).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    for i in &args[1..] {
      let txt = fs::read_to_string(i)?;
      fs::File::create(i)?.write_all(fmt(&txt).as_ref())?;
    }
  } else {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("{}", fmt(&input)))?;
  }

  Ok(())
}
