fn main() {
    reqwest::blocking::get("https://www.google.com/").unwrap();
}
