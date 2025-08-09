use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub fields: Vec<Field>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub validators: Vec<Validator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Date,
    Url,
    Email,
    Array(Box<FieldType>),
    Object(HashMap<String, FieldType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Validator {
    MinLength { value: usize },
    MaxLength { value: usize },
    Pattern { regex: String },
    Min { value: f64 },
    Max { value: f64 },
    Enum { values: Vec<Value> },
}

impl Schema {
    pub fn validate(&self, data: &Value) -> Result<()> {
        let obj = data.as_object()
            .ok_or_else(|| Error::Parse("Data must be an object".to_string()))?;

        for required_field in &self.required {
            if !obj.contains_key(required_field) {
                return Err(Error::Parse(format!(
                    "Required field '{}' is missing",
                    required_field
                )));
            }
        }

        for field in &self.fields {
            if let Some(value) = obj.get(&field.name) {
                self.validate_field(field, value)?;
            }
        }

        Ok(())
    }

    fn validate_field(&self, field: &Field, value: &Value) -> Result<()> {
        self.validate_type(&field.field_type, value)?;

        for validator in &field.validators {
            self.apply_validator(validator, value)?;
        }

        Ok(())
    }

    fn validate_type(&self, field_type: &FieldType, value: &Value) -> Result<()> {
        match (field_type, value) {
            (FieldType::String, Value::String(_)) => Ok(()),
            (FieldType::Number, Value::Number(_)) => Ok(()),
            (FieldType::Boolean, Value::Bool(_)) => Ok(()),
            (FieldType::Array(inner_type), Value::Array(arr)) => {
                for item in arr {
                    self.validate_type(inner_type, item)?;
                }
                Ok(())
            }
            (FieldType::Object(schema), Value::Object(obj)) => {
                for (key, expected_type) in schema {
                    if let Some(val) = obj.get(key) {
                        self.validate_type(expected_type, val)?;
                    }
                }
                Ok(())
            }
            _ => Err(Error::Parse(format!(
                "Type mismatch: expected {:?}, got {:?}",
                field_type, value
            ))),
        }
    }

    fn apply_validator(&self, validator: &Validator, value: &Value) -> Result<()> {
        match validator {
            Validator::MinLength { value: min } => {
                if let Value::String(s) = value {
                    if s.len() < *min {
                        return Err(Error::Parse(format!(
                            "String length {} is less than minimum {}",
                            s.len(),
                            min
                        )));
                    }
                }
            }
            Validator::MaxLength { value: max } => {
                if let Value::String(s) = value {
                    if s.len() > *max {
                        return Err(Error::Parse(format!(
                            "String length {} exceeds maximum {}",
                            s.len(),
                            max
                        )));
                    }
                }
            }
            Validator::Pattern { regex } => {
                if let Value::String(s) = value {
                    let re = regex::Regex::new(regex)
                        .map_err(|e| Error::Parse(format!("Invalid regex: {}", e)))?;
                    if !re.is_match(s) {
                        return Err(Error::Parse(format!(
                            "String '{}' does not match pattern '{}'",
                            s, regex
                        )));
                    }
                }
            }
            Validator::Min { value: min } => {
                if let Value::Number(n) = value {
                    if let Some(num) = n.as_f64() {
                        if num < *min {
                            return Err(Error::Parse(format!(
                                "Number {} is less than minimum {}",
                                num, min
                            )));
                        }
                    }
                }
            }
            Validator::Max { value: max } => {
                if let Value::Number(n) = value {
                    if let Some(num) = n.as_f64() {
                        if num > *max {
                            return Err(Error::Parse(format!(
                                "Number {} exceeds maximum {}",
                                num, max
                            )));
                        }
                    }
                }
            }
            Validator::Enum { values } => {
                if !values.contains(value) {
                    return Err(Error::Parse(format!(
                        "Value {:?} is not in allowed values {:?}",
                        value, values
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn normalize(&self, data: &mut Value) -> Result<()> {
        if let Value::Object(obj) = data {
            for field in &self.fields {
                if !obj.contains_key(&field.name) {
                    if let Some(default) = &field.default {
                        obj.insert(field.name.clone(), default.clone());
                    }
                }
            }
        }

        Ok(())
    }
}