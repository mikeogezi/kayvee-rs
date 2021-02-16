use std::collections::HashMap;
use std::io::Error;
use std::result::Result;

#[derive(Debug)]
pub struct Store {
  pub map: HashMap<String, String>,
  debug: bool,
}

impl juniper::Context for Store {}

impl Store {
  #[allow(dead_code)]
  pub fn new() -> Store {
    Store {
      map: HashMap::new(),
      debug: false,
    }
  }

  pub fn with_debug(debug: bool) -> Store {
    Store {
      map: HashMap::new(),
      debug,
    }
  }

  pub fn put(&mut self, key: &String, value: &String) -> Result<(), Error> {
    if self.debug {
      println!("Inserting value: '{}' into key: '{}'", value, key);
    }
    self.map.insert(key.to_owned(), value.to_owned());
    if self.debug {
      println!("Map: {:?}", self.map);
    }
    Ok(())
  }

  pub fn get(&mut self, key: &String) -> Result<Option<String>, Error> {
    if self.debug {
      println!("Getting value from key: '{}'", key);
    }
    let map = &self.map;
    let val = map.get(key);
    if self.debug {
      println!("Map: {:?}", self.map);
    }
    if val.is_some() {
      Ok(Some(val.unwrap().to_owned()))
    } else {
      Ok(None)
    }
  }

  pub fn delete(&mut self, key: &String) -> Result<(), Error> {
    if self.debug {
      println!("Deleting value from key: '{}'", key);
    }
    self.map.remove(key);
    if self.debug {
      println!("Map: {:?}", self.map);
    }
    Ok(())
  }
}
