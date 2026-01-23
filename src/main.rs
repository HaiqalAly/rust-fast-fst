use std::fs::File;
use std::io;

use fst::{IntoStreamer, Map, MapBuilder};
use fst::automaton::Levenshtein;

// Adapted from the fst crate examples by @burntsushi
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = "test.fst";
  let writer = io::BufWriter::new(File::create(path)?);
  let mut build = MapBuilder::new(writer)?;
  let mut keys = vec!["fa", "fo", "fob", "focus", "foo", "food", "foul"];
  keys.sort();

  for key in keys {
    build.insert(key, 0)?;
  }

  build.finish()?;

  let data = std::fs::read(path)?;
  let set = Map::new(data)?;
  let lev = Levenshtein::new("foo", 2)?;

  let stream = set.search(lev).into_stream();

  let keys = stream.into_str_keys()?;
  print!("{:#?}", keys);

  assert_eq!(keys, vec!["fa", "fo", "fob", "foo", "food", "foul"]);
  Ok(())
}