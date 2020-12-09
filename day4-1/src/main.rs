// Advent of Code — Day 3 Problem 1 Solution

use std::fs;
use std::collections::HashMap;

const INPUT_PATH: &str = "day4.input";
// const INPUT_PATH: &str = "fuck_chase.txt";

fn main() {
  let input = fs::read_to_string(INPUT_PATH).unwrap();

  let passports = input.split("\n\n");

  let mut valid = 0;
  for pass in passports {
    let pass = String::from(pass);
    match get_vals(&pass) {
      Err(_) => (),
      Ok(res) => {
        let byr = handle_4dig(&res["byr"], 1920, 2002);
        let iyr = handle_4dig(&res["iyr"], 2010, 2020);
        let eyr = handle_4dig(&res["eyr"], 2020, 2030);
        let hgt = handle_height(&res["hgt"]);
        let hcl = handle_hex(&res["hcl"]);
        let ecl = handle_ecl(&res["ecl"]);
        let pid = handle_pid(&res["pid"]);

        


        if byr && iyr && eyr && hgt && hcl && ecl && pid {
       	println!("byr:{}", &res["byr"]);
        println!("iyr:{}", &res["iyr"]);
        println!("eyr:{}", &res["eyr"]);
        println!("hgt:{}", &res["hgt"]);
        println!("hcl:{}", &res["hcl"]);
        println!("ecl:{}", &res["ecl"]);
        println!("pid:{}", &res["pid"]);
        println!();

          valid += 1;
        } else {
        	// println!("byr ({}): {}", &res["byr"], byr);
	        // println!("iyr ({}): {}", &res["iyr"], iyr);
	        // println!("eyr ({}): {}", &res["eyr"], eyr);
	        // println!("hgt ({}): {}", &res["hgt"], hgt);
	        // println!("hcl ({}): {}", &res["hcl"], hcl);
	        // println!("ecl ({}): {}", &res["ecl"], ecl);
         //    println!("pid ({}): {}", &res["pid"], pid);

        	// println!("INVALID!!!");
	        // println!("----------");

        }
      }
    }
  }

  println!("{}", valid);
}

fn handle_pid(s: &String) -> bool {
  let mut num_digits = 0;

  for c in s.chars() {
    if !("1234567890".contains(c)) {
      return false;
    }
    num_digits += 1;
  }

  num_digits == 9
}

fn handle_ecl(s: &String) -> bool {
  s.eq("amb") ||
  s.eq("blu") ||
  s.eq("brn") ||
  s.eq("gry") ||
  s.eq("grn") ||
  s.eq("hzl") ||
  s.eq("oth")
}

fn handle_hex(s: &String) -> bool {
  let mut num_chars = 0;
  for c in s.chars() {
    if num_chars == 0 {
      if c != '#' {
        return false;
      }
    } else {
      if !("1234567890abcdef".contains(c)) {
        println!("incorrect char: {}", c);
        return false;
      }
    }

    num_chars += 1;
  }

  num_chars == 7
}

fn handle_height(s: &String) -> bool {
  let mut min = 0;
  let mut max = 0;

  if s[s.len()-2..].eq("in") {
    min = 59;
    max = 76;
  } else if s[s.len()-2..].eq("cm") {
    min = 150;
    max = 193;
  }

  if min == 0 && max == 0 { return false; }

  let num_str = &s[..s.len()-2];

  match num_str.parse::<i32>() {
    Ok(n) => n >= min && n <= max,
    Err(_) => false
  }
}

fn handle_4dig(s: &String, min: i32, max: i32) -> bool {
  match s.parse::<i32>() {
    Ok(n) => n >= min && n <= max,
    Err(_) => false
  }
}

fn get_vals(pass: &String) -> Result<HashMap<String, String>, bool>  {
    if !pass.contains("byr") ||
       !pass.contains("iyr") ||
       !pass.contains("eyr") ||
       !pass.contains("hgt") ||
       !pass.contains("hcl") ||
       !pass.contains("ecl") ||
       !pass.contains("pid") {
        return Err(false);
    }

  let pass = pass.replace("\n", " ");
  let vals: Vec<&str> = pass.split(" ").collect();

  let mut pairs: HashMap<String, String> = HashMap::new();
  for val in vals {
  	if val.len() >= 2 {
	    let p: Vec<&str> = val.split(":").collect();
	    pairs.insert(p[0].to_string(), p[1].to_string());
	} else {
		println!("val: {}", val);
	}
  }

  Ok(pairs)
}
