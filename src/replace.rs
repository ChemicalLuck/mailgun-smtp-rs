use std::collections::HashMap;

struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    pub fn next_token(&mut self) -> Option<(String, bool)> {
        if self.content.is_empty() {
            return None;
        }
        if self.content[0] == '{' {
            self.chop_while(|c| *c == '{');
            let token = self.chop_while(|c| *c != '}').iter().collect::<String>();
            self.chop_while(|c| *c == '}');
            return Some((token, true));
        }

        return Some((self.chop_while(|c| *c != '{').iter().collect(), false));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (String, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Debug, PartialEq)]
pub struct ReplaceError {
    keys: Vec<String>,
}

impl ReplaceError {
    fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl std::fmt::Display for ReplaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self
            .keys
            .iter()
            .map(|k| format!("'{}'", k))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "Could not find keys: {}", keys)
    }
}

impl std::error::Error for ReplaceError {}

pub fn replace_variables(
    string: &str,
    variables: &HashMap<String, String>,
) -> Result<String, ReplaceError> {
    let chars = string.chars().collect::<Vec<char>>();
    let lexer = Lexer::new(&chars);
    let mut err_keys = Vec::new();
    let mut result = String::new();

    for (token, is_variable) in lexer {
        if is_variable {
            if let Some(value) = variables.get(&token) {
                result.push_str(value);
            } else {
                err_keys.push(token.clone());
            }
        } else {
            result.push_str(&token);
        }
    }

    if err_keys.is_empty() {
        Ok(result)
    } else {
        Err(ReplaceError::new(err_keys))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_variables_valid() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("age".to_string(), "25".to_string());

        let input = "Hello, {name}! You are {age} years old.";
        let result = replace_variables(input, &variables);
        assert_eq!(result, Ok("Hello, John! You are 25 years old.".to_string()));
    }

    #[test]
    fn test_replace_variables_missing() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("age".to_string(), "25".to_string());

        let input_missing = "Hello, {name}! You are {height} years old.";
        let result_missing = replace_variables(input_missing, &variables);
        assert_eq!(
            result_missing,
            Err(ReplaceError::new(vec!["height".to_string()]))
        );
    }

    #[test]
    fn test_replace_variables_nested() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("age".to_string(), "25".to_string());

        let input_nested = "Nested: {{name}} - {age}";
        let result_nested = replace_variables(input_nested, &variables);
        assert_eq!(result_nested, Ok("Nested: John - 25".to_string()));
    }
}
