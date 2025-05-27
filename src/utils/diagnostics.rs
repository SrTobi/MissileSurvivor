use std::collections::btree_map::Entry as BTEntry;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::Display;
use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;

lazy_static! {
  pub static ref DIAGNOSTICS: Mutex<Diagnostics> = Mutex::new(Diagnostics::new());
}

enum Data {
  Number { values: VecDeque<f64>, unit: &'static str },
  String(String),
}

impl Display for Data {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Data::Number { values, unit, .. } => {
        let values_it = values.iter().copied();
        let mean = values_it.clone().sum::<f64>() / values.len() as f64;
        let min = values_it.clone().reduce(f64::min).unwrap();
        let max = values_it.clone().reduce(f64::max).unwrap();
        write!(f, "{mean:.2}{unit} ({min:.1}{unit}-{max:.1}{unit})")
      }
      Data::String(value) => write!(f, "{value}"),
    }
  }
}

struct Entry {
  data: Data,
  epoch_t: f64,
}

pub struct Diagnostics {
  entries: Mutex<BTreeMap<String, Entry>>,
  timings: Mutex<HashMap<&'static str, Measurement>>,
  epoch_t: f64,
}

impl Diagnostics {
  fn new() -> Self {
    Self {
      entries: Mutex::new(BTreeMap::new()),
      timings: Mutex::new(HashMap::new()),
      epoch_t: macroquad::time::get_time(),
    }
  }

  pub fn get() -> MutexGuard<'static, Self> {
    DIAGNOSTICS.lock().unwrap()
  }

  pub fn report(&self, name: &str, value: &str) {
    self.insert(name.to_string(), Data::String(value.to_string()));
  }

  pub fn report_number(&self, name: &str, value: impl Into<f64>, unit: &'static str) {
    self.insert_number(name.to_string(), value.into(), unit);
  }

  pub fn report_measurement(&self, value: Measurement) {
    let elapsed = macroquad::time::get_time() - value.start_time;

    self.insert_number(value.name, elapsed * 1000., "ms");
  }

  pub fn start_timing(&self, name: &'static str) {
    self.timings.lock().unwrap().insert(name, Measurement::new(name));
  }

  pub fn end_timing(&self, name: &str) {
    let start_time = self.timings.lock().unwrap().remove(name).unwrap();
    self.report_measurement(start_time);
  }

  fn insert_number(&self, name: String, value: f64, unit: &'static str) {
    let make_default = || Entry {
      data: Data::Number {
        values: [value].into_iter().collect(),
        unit,
      },
      epoch_t: self.epoch_t,
    };
    let mut entries = self.entries.lock().unwrap();
    match entries.entry(name) {
      BTEntry::Vacant(entry) => {
        entry.insert(make_default());
      }
      BTEntry::Occupied(entry) => {
        let Entry { data, epoch_t } = entry.into_mut();
        *epoch_t = self.epoch_t;
        match data {
          Data::Number { values, .. } => {
            values.push_back(value);
            while values.len() > 32 {
              values.pop_front();
            }
          }
          Data::String(_) => {
            *data = make_default().data;
          }
        }
      }
    }
  }

  fn insert(&self, name: String, data: Data) {
    self.entries.lock().unwrap().insert(
      name,
      Entry {
        data: data,
        epoch_t: self.epoch_t,
      },
    );
  }

  pub fn update() {
    let mut diag = Diagnostics::get();
    diag.epoch_t = macroquad::time::get_time();

    diag.entries.lock().unwrap().retain(|_, entry| {
      // keep old entries for 10s
      entry.epoch_t + 10. > diag.epoch_t
    });
  }

  pub fn render() {
    let diag: MutexGuard<'_, Diagnostics> = Diagnostics::get();
    let text_size: f32 = 16.;
    let line_size = text_size + 2.;
    let text_offset: f32 = 5.;

    let map = diag.entries.lock().unwrap();
    let mut y = text_offset + text_size / 2.;
    for (name, entry) in map.iter() {
      let Entry { data, epoch_t } = entry;

      let old = if *epoch_t + 0.5 >= diag.epoch_t {
        String::new()
      } else {
        format!("({:.1}ms old)", (diag.epoch_t - epoch_t) * 1000.)
      };

      let text = format!("{name}: {data} {old}");
      macroquad::text::draw_text(&text, text_offset, y, text_size, macroquad::color::RED);
      y += line_size;
    }
  }
}

pub struct Measurement {
  name: String,
  start_time: f64,
}

impl Measurement {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      start_time: macroquad::time::get_time(),
    }
  }
}

#[macro_export]
macro_rules! timing_system_start {
  ($group:ident / $name:ident) => {{
    paste::paste! {
        fn [<start_timing_ $group _ $name >] (diag: bevy_ecs::system::ResMut<Diagnostics>) {
        diag.start_timing(concat!(stringify!($group), "/", stringify!($name)));
      }

      [< start_timing_ $group _ $name >]
    }
  }};
}

#[macro_export]
macro_rules! timing_system_end {
  ($group:ident / $name:ident) => {{
    paste::paste! {
        fn [<start_timing_ $group _ $name >] (diag: bevy_ecs::system::ResMut<Diagnostics>) {
        diag.end_timing(concat!(stringify!($group), "/", stringify!($name)));
      }

      [< start_timing_ $group _ $name >]
    }
  }};
}
