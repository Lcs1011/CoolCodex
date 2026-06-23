use crate::error::CToolError;
use crate::error::CToolResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SymbolRange {
    pub kind: String,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub signature: String,
}

pub(crate) fn outline_symbols(text: &str) -> Vec<SymbolRange> {
    let lines = text.lines().collect::<Vec<_>>();
    let mut symbols = Vec::new();
    let mut pending_test = false;

    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim_start();

        if trimmed.starts_with("#[test]") || trimmed.starts_with("#[tokio::test]") {
            pending_test = true;
            continue;
        }
        if pending_test && trimmed.starts_with("#[") {
            continue;
        }

        let Some((kind, name)) = parse_symbol_header(trimmed, pending_test) else {
            pending_test = false;
            continue;
        };
        pending_test = false;

        let end_line = find_symbol_end_line(&lines, index);
        symbols.push(SymbolRange {
            kind,
            name,
            start_line: line_number,
            end_line,
            signature: trimmed.to_string(),
        });
    }

    symbols
}

pub(crate) fn read_lines_range(
    text: &str,
    start_line: usize,
    end_line: usize,
) -> CToolResult<String> {
    if start_line == 0 {
        return Err(CToolError::InvalidInput(
            "start_line must be greater than 0".to_string(),
        ));
    }
    if end_line < start_line {
        return Err(CToolError::InvalidInput(
            "end_line must be greater than or equal to start_line".to_string(),
        ));
    }

    let lines = text.lines().collect::<Vec<_>>();
    if start_line > lines.len() {
        return Err(CToolError::InvalidInput(format!(
            "start_line {start_line} is greater than total line count {}",
            lines.len()
        )));
    }

    Ok(lines[(start_line - 1)..end_line.min(lines.len())].join("\n"))
}

fn parse_symbol_header(trimmed: &str, pending_test: bool) -> Option<(String, String)> {
    let normalized = trim_rust_prefixes(trimmed);

    if let Some(name) = parse_named_item(normalized, "struct") {
        return Some(("struct".to_string(), name));
    }
    if let Some(name) = parse_named_item(normalized, "enum") {
        return Some(("enum".to_string(), name));
    }
    if let Some(name) = parse_named_item(normalized, "mod") {
        return Some(("mod".to_string(), name));
    }
    if let Some(name) = parse_named_item(normalized, "fn") {
        return Some(((if pending_test { "test" } else { "fn" }).to_string(), name));
    }

    let impl_header = if normalized.starts_with("impl ") {
        Some(normalized)
    } else if let Some(rest) = normalized.strip_prefix("unsafe impl ") {
        Some(rest)
    } else {
        None
    }?;

    Some(("impl".to_string(), clean_impl_name(impl_header)))
}

fn trim_rust_prefixes(mut text: &str) -> &str {
    loop {
        let old = text;
        for prefix in [
            "pub ",
            "pub(crate) ",
            "pub(super) ",
            "pub(in crate) ",
            "async ",
            "const ",
            "unsafe ",
            "extern ",
        ] {
            if let Some(rest) = text.strip_prefix(prefix) {
                text = rest.trim_start();
            }
        }
        if text == old {
            return text;
        }
    }
}

fn parse_named_item(text: &str, keyword: &str) -> Option<String> {
    let rest = text.strip_prefix(keyword)?.trim_start();
    if rest.is_empty() {
        return None;
    }

    let name = rest
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect::<String>();
    if name.is_empty() { None } else { Some(name) }
}

fn clean_impl_name(header: &str) -> String {
    header
        .trim_end_matches('{')
        .trim()
        .trim_start_matches("impl")
        .trim()
        .to_string()
}

fn find_symbol_end_line(lines: &[&str], start_index: usize) -> usize {
    let mut balance = 0isize;
    let mut saw_open = false;

    for (index, line) in lines.iter().enumerate().skip(start_index) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    saw_open = true;
                    balance += 1;
                }
                '}' if saw_open => {
                    balance -= 1;
                    if balance <= 0 {
                        return index + 1;
                    }
                }
                _ => {}
            }
        }

        if !saw_open && line.trim_end().ends_with(';') {
            return index + 1;
        }
    }

    start_index + 1
}
