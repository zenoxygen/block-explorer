use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use rocket_dyn_templates::tera::{
    Error as TeraError, Filter, Result as TeraResult, Value as TeraValue,
};

pub struct FormatHashFilter;

impl Filter for FormatHashFilter {
    fn filter(
        &self,
        value: &TeraValue,
        _args: &HashMap<String, TeraValue>,
    ) -> TeraResult<TeraValue> {
        let hash = value
            .as_str()
            .ok_or_else(|| TeraError::msg("failed to parse value as string"))?;

        if hash.len() < 16 {
            return Err(TeraError::msg("hash is too short to format"));
        }

        let hash_start = &hash[..8];
        let hash_end = &hash[hash.len() - 8..];
        let formatted_hash = format!("{}...{}", hash_start, hash_end);

        Ok(TeraValue::String(formatted_hash))
    }
}

pub struct FormatTimeFilter;

impl Filter for FormatTimeFilter {
    fn filter(
        &self,
        value: &TeraValue,
        _args: &HashMap<String, TeraValue>,
    ) -> TeraResult<TeraValue> {
        let timestamp = value
            .as_i64()
            .ok_or_else(|| TeraError::msg("failed to parse value as string"))?;

        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .ok_or_else(|| TeraError::msg("failed to create datetime from timestamp"))?;

        let datetime = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        Ok(TeraValue::String(formatted_time))
    }
}

pub struct FormatNumberFilter;

impl Filter for FormatNumberFilter {
    fn filter(
        &self,
        value: &TeraValue,
        args: &HashMap<String, TeraValue>,
    ) -> TeraResult<TeraValue> {
        let rounded = match args.get("rounded") {
            Some(val) => val.as_i64().unwrap_or(0) as usize,
            None => 0,
        };

        match value {
            TeraValue::Number(n) => {
                if n.is_i64() {
                    let int_value_parsed = n.as_i64().unwrap();
                    Ok(TeraValue::String(format_int(int_value_parsed)))
                } else if n.is_f64() {
                    let float_value_parsed = n.as_f64().unwrap();
                    Ok(TeraValue::String(format_float(float_value_parsed, rounded)))
                } else {
                    Err(TeraError::msg("unsupported number type"))
                }
            }
            _ => Err(TeraError::msg("failed to parse value as number")),
        }
    }
}

fn format_int(value: i64) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut count = 0;
    for ch in value.to_string().chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(ch);
        count += 1;
    }

    result.chars().rev().collect()
}

fn format_float(value: f64, rounded: usize) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    if rounded == 0 {
        let int_part = value.trunc() as i64;
        return format_int(int_part);
    }

    let s_value = format!("{:.*}", rounded, value);

    let parts: Vec<&str> = s_value.split('.').collect();
    let int_part = format_int(parts[0].parse().unwrap());

    format!("{}.{}", int_part, parts[1])
}
