pub struct Path {
  pub path_prefix: String,
}

impl Path {
  pub fn define_path(&self, following_path: &str) -> String {
    self.path_prefix.to_owned() + &following_path
  }
}
