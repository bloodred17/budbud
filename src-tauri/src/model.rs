use std::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub first: Cow<'static, str>,
    pub last: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub title: Cow<'static, str>,
    pub name: Name,
    pub marketing: bool,
}