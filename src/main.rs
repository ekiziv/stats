extern crate regex;
use regex::Regex;
use regex::RegexSetBuilder;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
  let buffered = BufReader::new(File::open("old_100_release.txt")?);
  let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("old_100_release_current_state_cloned")
    .unwrap();

  let set = RegexSetBuilder::new(&[
    r#"starting migration"#,
    r#"current state cloned for replay"#,
  ])
  .case_insensitive(true)
  .build()?;
  let re = Regex::new(r"μs: (\d{0,10})").unwrap();
  let mut res: String = "".to_owned();

  buffered
    .lines()
    .filter_map(|line| line.ok())
    .filter(|line| set.is_match(line.as_str()))
    .for_each(|x| res.push_str(&format!("{}\n", x)));

  // prints to see if the collected data makes sense
  println!("{}", res);
  let mut person_count = 0;

  let mut count = 0;
  for line in res.lines() {
    if line.contains("starting migration") {

      // Some of the migration do not contain interesting info. Meaningful data
      // comes across every 3 rows in some cases. Then, adding 0 at the first
      // line and then collecting every third line extracts meaningful data.
      // See later where every third line is stored.
      if person_count == 0 {
        let _err = write!(&mut file, "0\n");
      }

      let _err = write!(&mut file, "{}\n", &count.clone());
      person_count = person_count + 1;
      count = 0;
    } else {
      for cap in re.captures_iter(line) {
        count += &cap[1].parse().unwrap();
      }
    }
  }
  // this is where only meaningful data is extacted. Sometimes may be unnecessary.
  let mut meaningful: String = "".to_owned();
  let buff = BufReader::new(File::open("old_100_release_current_state_cloned")?);
  let mut buff_str: String = "".to_owned();

  buff
    .lines()
    .filter_map(|line| line.ok())
    .for_each(|x| buff_str.push_str(&format!("{}\n", x)));

  let mut i = 0;
  for line in buff_str.lines() {
    // first and second lines turn out to be meaningful
    if i == 1 || i == 2 {
      meaningful.push_str(&format!("{}\n", line));
    }
    if i % 3 == 0 && i != 0 {
      meaningful.push_str(&format!("{}\n", line));
    }
    i += 1;
  }
  println!("{}", meaningful);

  Ok(())
}
