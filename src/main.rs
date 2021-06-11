extern crate glob;

use glob::glob;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const DIRECTORY_TO_SCAN: &'static str = "DIRECTORY_TO_SCAN";
const FILE_EXTENSION_TO_SCAN: &'static str = "FILE_EXTENSION_TO_SCAN";
const SENTENCE_TO_MATCH: &'static str = "SENTENCE_TO_MATCH";

fn main() {
  assert!(!DIRECTORY_TO_SCAN.ends_with("/"), "O diretório para scanear não pode terminar com /");

  let scan_path = format!("{}/*.{}", DIRECTORY_TO_SCAN, FILE_EXTENSION_TO_SCAN);

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
