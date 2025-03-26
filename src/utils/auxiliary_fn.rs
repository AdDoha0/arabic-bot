

pub fn escape_markdown(text: &str) -> String {
    let reserved = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#',
        '+', '-', '=', '|', '{', '}', '.', '!', ':', '\\' // Добавил `:` и `\`
    ];
    let mut escaped = String::with_capacity(text.len());

    for c in text.chars() {
        if reserved.contains(&c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }

    escaped
}

