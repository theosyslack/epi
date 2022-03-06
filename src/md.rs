pub struct Md {
    
}

impl Md {
    pub fn title(str: &str) -> String {
        format!("# {}\n", str)
    }

    pub fn ol(num: usize, str: &str) -> String {
        format!("{}. {}\n", num, str)
    }

    pub fn img(alt: &str, src: &str) -> String {
        format!("![{}]({})\n", alt, src)
    }
}