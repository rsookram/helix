use std::collections::HashMap;

#[derive(Debug)]
pub struct Register {
    name: char,
    values: Vec<String>,
}

impl Register {
    pub fn new(name: char) -> Self {
        Self {
            name,
            values: Vec::new(),
        }
    }

    pub fn new_with_values(name: char, values: Vec<String>) -> Self {
        Self { name, values }
    }

    pub fn name(&self) -> char {
        self.name
    }

    pub fn read(&self) -> &[String] {
        &self.values
    }

    pub fn write(&mut self, values: Vec<String>) {
        self.values = values;
    }

    pub fn push(&mut self, value: String) {
        self.values.push(value);
    }
}

/// Currently just wraps a `HashMap` of `Register`s
#[derive(Debug, Default)]
pub struct Registers {
    inner: HashMap<char, Register>,
}

impl Registers {
    pub fn get(&self, name: char) -> Option<&Register> {
        self.inner.get(&name)
    }

    pub fn get_mut(&mut self, name: char) -> &mut Register {
        self.inner
            .entry(name)
            .or_insert_with(|| Register::new(name))
    }

    pub fn write(&mut self, name: char, values: Vec<String>) {
        self.inner
            .insert(name, Register::new_with_values(name, values));
    }

    pub fn read(&self, name: char) -> Option<&[String]> {
        self.get(name).map(|reg| reg.read())
    }
}
