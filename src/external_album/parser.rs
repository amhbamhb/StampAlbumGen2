use super::{Argument, LayoutCommand, LayoutPage, Result};

fn numeric(command: &LayoutCommand, index: usize, positive: bool) -> Result<f64> {
    let argument = command.args.get(index).ok_or("Missing numeric argument")?;
    let value = argument
        .value
        .parse::<f64>()
        .map_err(|_| format!("{} argument {} must be numeric", command.name, index + 1))?;
    if argument.quoted || !value.is_finite() || (positive && value <= 0.0) {
        return Err(format!(
            "{} argument {} has an invalid numeric value",
            command.name,
            index + 1
        ));
    }
    Ok(value)
}

pub(super) fn number(command: &LayoutCommand, index: usize) -> Result<f64> {
    numeric(command, index, false)
}

pub(super) fn validate(command: &LayoutCommand) -> Result<()> {
    let (min, max, numbers, strings): (usize, usize, &[usize], &[usize]) =
        match command.name.as_str() {
            "ALBUM_PAGES_SIZE" => (2, 2, &[0, 1], &[]),
            "ALBUM_PAGES_MARGINS" | "ALBUM_PAGES_MARGINSE" => (4, 4, &[0, 1, 2, 3], &[]),
            "ALBUM_PAGES_BORDER" => (3, 4, &[0, 1, 2, 3], &[]),
            "ALBUM_PAGES_SPACING" => (2, 2, &[0, 1], &[]),
            "ALBUM_PAGES_TITLE" => (3, 3, &[1], &[2]),
            "ALBUM_PAGES_NUMBER" => (3, 6, &[1, 3], &[4, 5]),
            "STAMP_BOXES_SIZE_ADJUST" => (1, 1, &[0], &[]),
            "ALBUM_STAMP_IMG_SETTING" => (3, 3, &[0, 1], &[]),
            "ROW_ALIGN_BOTTOM" | "PAGE_START" => (0, 0, &[], &[]),
            "PAGE_TEXT_CENTRE" => (3, 4, &[1, 3], &[2]),
            "ROW_START_FS" => (4, 4, &[1, 2, 3], &[]),
            "STAMP_ADD_IMG" => (6, 7, &[0, 1], &[2, 3, 4, 5]),
            _ => return Err(format!("Unsupported command {}", command.name)),
        };
    if !(min..=max).contains(&command.args.len()) {
        return Err(format!(
            "{} expects {min}..{max} arguments, received {}",
            command.name,
            command.args.len()
        ));
    }
    for &index in numbers.iter().filter(|&&i| i < command.args.len()) {
        let positive = matches!(command.name.as_str(), "ALBUM_PAGES_SIZE" | "STAMP_ADD_IMG")
            || (index == 1
                && matches!(
                    command.name.as_str(),
                    "ROW_START_FS"
                        | "PAGE_TEXT_CENTRE"
                        | "ALBUM_PAGES_TITLE"
                        | "ALBUM_PAGES_NUMBER"
                ));
        let n = numeric(command, index, positive)?;
        if n < 0.0 && command.name != "STAMP_BOXES_SIZE_ADJUST" {
            return Err(format!(
                "{} argument {} must be nonnegative",
                command.name,
                index + 1
            ));
        }
    }
    for &index in strings.iter().filter(|&&i| i < command.args.len()) {
        if !command.args[index].quoted {
            return Err(format!(
                "{} argument {} must be quoted",
                command.name,
                index + 1
            ));
        }
    }
    for argument in &command.args {
        if argument.value.contains(['\0', '\r', '\n']) {
            return Err("Literal line breaks/control characters are not supported; use the AlbumEasy \\n escape".into());
        }
        if argument.quoted {
            let mut escaped = false;
            for c in argument.value.chars() {
                if c == '"' && !escaped {
                    return Err("Unescaped quote in argument".into());
                }
                escaped = c == '\\' && !escaped;
            }
            if escaped {
                return Err("Quoted argument ends with an incomplete escape".into());
            }
        } else if argument.value.is_empty()
            || argument
                .value
                .chars()
                .any(|c| c.is_whitespace() || "()#;\"".contains(c))
        {
            return Err("Invalid unquoted argument".into());
        }
    }
    if command.name == "ALBUM_STAMP_IMG_SETTING"
        && !matches!(
            command.args[2].value.to_ascii_lowercase().as_str(),
            "true" | "false"
        )
    {
        return Err("Image stretch must be True or False".into());
    }
    if command.name == "ALBUM_PAGES_NUMBER" {
        if !matches!(
            command.args[2].value.as_str(),
            "L" | "R" | "C" | "LR" | "RL"
        ) {
            return Err("Invalid page-number alignment".into());
        }
        if command.args.len() > 3 && command.args[3].value.parse::<u32>().is_err() {
            return Err("Page numbering start must be a nonnegative integer".into());
        }
    }
    if command.name == "STAMP_ADD_IMG"
        && command.args.len() == 7
        && command.args[6].value != "NO_FRAME"
    {
        return Err("Only NO_FRAME is supported as the optional stamp image argument".into());
    }
    Ok(())
}

pub(super) fn validate_page(page: &LayoutPage) -> Result<()> {
    if page.commands.first().is_none_or(|c| c.name != "PAGE_START") {
        return Err(format!("Page {} must begin with PAGE_START", page.index));
    }
    let mut row = false;
    for (index, command) in page.commands.iter().enumerate() {
        validate(command).map_err(|e| format!("Line {}: {e}", command.source_line))?;
        match command.name.as_str() {
            "PAGE_START" if index != 0 => {
                return Err("Editing cannot add/remove page boundaries".into())
            }
            "ROW_START_FS" => row = true,
            "STAMP_ADD_IMG" if !row => {
                return Err("STAMP_ADD_IMG must follow ROW_START_FS on its page".into())
            }
            "PAGE_TEXT_CENTRE" => row = false,
            "PAGE_START" | "STAMP_ADD_IMG" | "ROW_ALIGN_BOTTOM" => {}
            _ => {
                return Err(format!(
                    "{} is a document setting, not an editable page command",
                    command.name
                ))
            }
        }
    }
    Ok(())
}

pub(super) fn validate_document(globals: &[LayoutCommand], pages: &[LayoutPage]) -> Result<()> {
    for command in globals {
        validate(command)?;
        if matches!(
            command.name.as_str(),
            "PAGE_START" | "PAGE_TEXT_CENTRE" | "ROW_START_FS" | "STAMP_ADD_IMG"
        ) {
            return Err(format!("{} cannot precede the first page", command.name));
        }
    }
    for required in [
        "ALBUM_PAGES_SIZE",
        "ALBUM_PAGES_MARGINS",
        "ALBUM_PAGES_SPACING",
    ] {
        if !globals.iter().any(|c| c.name == required) {
            return Err(format!("Missing required command {required}"));
        }
    }
    if pages.is_empty() {
        return Err("At least one PAGE_START is required".into());
    }
    for (index, page) in pages.iter().enumerate() {
        if page.index != index + 1 {
            return Err("Page indexes must be contiguous and one-based; zero is reserved for global settings".into());
        }
        validate_page(page)?;
    }
    Ok(())
}

pub(super) fn parse(text: &str) -> (Vec<LayoutCommand>, Vec<LayoutPage>, Vec<String>) {
    let mut globals = Vec::new();
    let mut pages: Vec<LayoutPage> = Vec::new();
    let mut errors = Vec::new();
    let mut lexer = Lexer {
        chars: text.trim_start_matches('\u{feff}').chars().collect(),
        offset: 0,
        line: 1,
    };
    while lexer.skip() {
        let line = lexer.line;
        match lexer.command() {
            Ok(mut command) => {
                command.source_line = line;
                if let Err(error) = validate(&command) {
                    errors.push(format!("Line {line}: {error}"));
                }
                if command.name == "PAGE_START" {
                    pages.push(LayoutPage {
                        index: pages.len() + 1,
                        commands: vec![command],
                    });
                } else if let Some(page) = pages.last_mut() {
                    page.commands.push(command);
                } else {
                    globals.push(command);
                }
            }
            Err(error) => {
                errors.push(format!("Line {line}: {error}"));
                while lexer.peek().is_some_and(|c| c != '\n') {
                    lexer.bump();
                }
            }
        }
    }
    if errors.is_empty() {
        if let Err(error) = validate_document(&globals, &pages) {
            errors.push(error);
        }
    }
    (globals, pages, errors)
}

struct Lexer {
    chars: Vec<char>,
    offset: usize,
    line: usize,
}

impl Lexer {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.offset).copied()
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.offset += 1;
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }
    fn skip(&mut self) -> bool {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() || c == ';' => {
                    self.bump();
                }
                Some('#') => {
                    while self.peek().is_some_and(|c| c != '\n') {
                        self.bump();
                    }
                }
                _ => return self.peek().is_some(),
            }
        }
    }
    fn command(&mut self) -> Result<LayoutCommand> {
        let mut name = String::new();
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_uppercase() || c == '_')
        {
            name.push(self.bump().ok_or("Unexpected EOF")?);
        }
        if name.is_empty() {
            return Err("Expected an uppercase AlbumEasy command".into());
        }
        while self
            .peek()
            .is_some_and(|c| c == ' ' || c == '\t' || c == '\r')
        {
            self.bump();
        }
        let mut args = Vec::new();
        if self.peek() == Some('(') {
            self.bump();
            loop {
                while self.peek().is_some_and(char::is_whitespace) {
                    self.bump();
                }
                match self.peek() {
                    Some(')') => {
                        self.bump();
                        break;
                    }
                    None => return Err(format!("Unterminated argument list for {name}")),
                    Some('"') => {
                        self.bump();
                        let mut value = String::new();
                        let mut escaped = false;
                        loop {
                            match self.bump() {
                                Some('"') if !escaped => break,
                                Some(c) => {
                                    value.push(c);
                                    escaped = c == '\\' && !escaped;
                                }
                                None => {
                                    return Err(format!("Unterminated quoted argument for {name}"))
                                }
                            }
                        }
                        args.push(Argument {
                            value,
                            quoted: true,
                        });
                        if self.peek().is_some_and(|c| !c.is_whitespace() && c != ')') {
                            return Err("Arguments must be separated by whitespace".into());
                        }
                    }
                    Some('#' | '(') => return Err("Unexpected character inside arguments".into()),
                    Some(_) => {
                        let mut value = String::new();
                        while self.peek().is_some_and(|c| !c.is_whitespace() && c != ')') {
                            value.push(self.bump().ok_or("Unexpected EOF")?);
                        }
                        args.push(Argument {
                            value,
                            quoted: false,
                        });
                    }
                }
            }
        }
        if self
            .peek()
            .is_some_and(|c| !c.is_whitespace() && c != ';' && c != '#')
        {
            return Err(format!("Unexpected trailing input after {name}"));
        }
        Ok(LayoutCommand {
            name,
            args,
            source_line: 0,
        })
    }
}

pub(super) fn serialize(globals: &[LayoutCommand], pages: &[LayoutPage]) -> Result<String> {
    validate_document(globals, pages)?;
    let mut output = String::new();
    for command in globals.iter().chain(pages.iter().flat_map(|p| &p.commands)) {
        output.push_str(&command.name);
        if !command.args.is_empty() {
            output.push_str(" (");
            for (index, argument) in command.args.iter().enumerate() {
                if index != 0 {
                    output.push(' ');
                }
                if argument.quoted {
                    output.push('"');
                }
                output.push_str(&argument.value);
                if argument.quoted {
                    output.push('"');
                }
            }
            output.push(')');
        }
        output.push('\n');
    }
    Ok(output)
}
