pub fn text<T>(text: T) -> Text
where
    T: Into<String>,
{
    Text::new(text)
}

#[derive(Clone)]
pub struct Text {
    pub value: String,
}

impl Text {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { value: text.into() }
    }
}

impl From<&str> for Text {
    fn from(from: &str) -> Self {
        Self {
            value: from.to_owned(),
        }
    }
}

impl From<String> for Text {
    fn from(from: String) -> Self {
        Self { value: from }
    }
}
