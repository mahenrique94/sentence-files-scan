extern crate glob;

use glob::glob;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const DIRECTORIES_TO_SCAN: [&'static str; 1] = ["DIRECTORIES_TO_SCAN"];
const FILE_EXTENSION_TO_SCAN: &'static str = "FILE_EXTENSION_TO_SCAN";
const LOGS_PATH: &'static str = "LOGS_PATH";
const SENTENCE_TO_MATCH: &'static str = "SENTENCE_TO_MATCH";
const SHOW_FILE_CONTENT: bool = true;

fn main() {
  assert!(!LOGS_PATH.ends_with("/"), "O diretório para scanear não pode terminar com /");

  for directory in DIRECTORIES_TO_SCAN.iter() {
    let scan_path = format!("{}/{}/*.{}", LOGS_PATH, directory, FILE_EXTENSION_TO_SCAN);
  
    for entry in glob(&scan_path).expect("Não conseguiu processador o glob pattern") {
      match entry {
        Ok(path) => {
          let file = File::open(path.clone()).unwrap();
          let reader = BufReader::new(file);
  
          for line in reader.lines() {
            match line {
              Ok(content) => {
                if content.contains(SENTENCE_TO_MATCH) {
                  println!("O arquivo [{:#?}] contém a sentença [{:?}]", path.clone(), SENTENCE_TO_MATCH);
                  if SHOW_FILE_CONTENT {
                    println!("\n{:#?}\n", content);
                  }
                }
              },
              Err(e2) => println!("{:#?}", e2),
            }
          }  
        },
        Err(e1) => println!("{:#?}", e1),
      }
    }
  }
}
