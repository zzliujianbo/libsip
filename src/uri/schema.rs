use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Schema {
    Sip,
    Sips
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schema::Sip => write!(f, "sip"),
            Schema::Sips => write!(f, "sips")
        }
    }
}


named!(pub parse_schema<Schema>, alt!(
    map!(tag!("sip"), |_| Schema::Sip) |
    map!(tag!("sips"), |_| Schema::Sips)
));
