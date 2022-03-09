pub struct Path {
  pub prefix: String,
}

impl Path {
  pub fn new(input_prefix: &str) -> Self {
    Self {
      prefix: input_prefix.to_string(),
    }
  }
  pub fn define(&self, following_path: &str) -> String {
    self.prefix.to_owned() + following_path
  }
}
