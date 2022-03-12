use std::fs;

pub fn read_file(file_name: &str) -> String {
  println!("{}", &file_name);
  fs::read_to_string(file_name).expect("could not open file")
}

pub fn add_component(component_tag: &str, html_data: &str) -> String {
  let css_tag = component_tag.to_uppercase() + "_CSS";
  let html_tag = component_tag.to_uppercase() + "_HTML";

  let css_path = format!(
    "{}{}{}",
    "./templates/components/",
    &component_tag.to_lowercase(),
    ".css"
  );
  let css_loaded = read_file(&css_path);

  let html_path = format!(
    "{}{}{}",
    "./templates/components/",
    &component_tag.to_lowercase(),
    ".html"
  );
  let html_loaded = read_file(&html_path);

  let mut html_data = html_data.replace(html_tag.as_str(), &html_loaded);
  html_data = html_data.replace(css_tag.as_str(), &css_loaded);
  html_data
}
