use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    name: String,
    items: Vec<String>,
    current_item: usize,
}

impl Todo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
            current_item: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.items.is_empty() {
            return None;
        }
        
        self.current_item = (self.current_item + 1) % self.items.len();
        let item = &self.items[self.current_item]; 
        
        Some(item) 
    }
    
    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        if index == 0 || index > self.items.len() {
            return Err("Index out of range".to_string());
        }

        let i = index - 1;
        self.items.remove(i);

        // En los otros casos el current_item no se ve afectado
        if i < self.current_item {
            self.current_item -= 1;
        } else if i == self.current_item && self.current_item >= self.items.len() {
            self.current_item = 0;
        }

        Ok(())
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.items.is_empty() {
            writeln!(f, "(empty list)")?;
        } else {
            for (i, item) in self.items.iter().enumerate() {
                if i == self.current_item {
                    writeln!(f, "> {}. {}", i + 1, item)?;
                } else {
                    writeln!(f, "  {}. {}", i + 1, item)?;
                }
            }
        }
        Ok(())
    }
}