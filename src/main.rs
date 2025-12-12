use std::{
    collections::HashMap,
    io::{stdin, stdout},
};

use serde::{Deserialize, Serialize};

mod rules;

const HELP: &str =
    "Convert VSCode color themes into Helix themes.\nPipe the file into this through stdin.\n";

fn main() {
    let raw_args = clap_lex::RawArgs::new(std::env::args_os());
    let mut argc = raw_args.cursor();
    #[allow(clippy::never_loop)]
    while let Some(arg) = raw_args.next(&mut argc) {
        if arg.is_empty() {
            continue;
        }
        if let Some(args) = arg.to_short() {
            for c in args.filter_map(Result::ok) {
                match c {
                    'h' => {
                        print!("{HELP}");
                        return;
                    }
                    _ => {
                        eprintln!("unknown argument '-{c}'.");
                        return;
                    }
                }
            }
        }
        if let Some((Ok(a), _)) = arg.to_long() {
            match a {
                "help" => {
                    print!("{HELP}");
                    return;
                }
                _ => {
                    eprintln!("unknown argument '--{a}'.");
                    return;
                }
            }
        }
    }

    let code_theme: CodeTheme = serde_json::from_reader(stdin()).unwrap();
    let mut hx_theme = HelixTheme {
        colors: HashMap::new(),
        rainbow: vec![],
    };
    rules::write(&code_theme, &mut hx_theme);
    let mut stdout = stdout();
    use std::io::Write;
    write!(
        stdout,
        "{}",
        toml::to_string_pretty(&hx_theme).expect("failed to serialize into TOML")
    )
    .unwrap();
}

#[derive(Serialize)]
struct HelixTheme {
    #[serde(flatten)]
    colors: HashMap<&'static str, helix_color::Entry>,
    rainbow: Vec<helix_color::Entry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeTheme {
    colors: HashMap<Box<str>, Option<Box<str>>>,
    token_colors: Box<[token_color::TokenColor]>,
}

mod helix_color {
    use std::collections::HashSet;

    use serde::Serialize;

    #[derive(Serialize, Default, Clone)]
    pub struct Entry {
        // permitted to be `None` in some special places, e.g. `ui.text.focus`
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fg: Option<Box<str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bg: Option<Box<str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub underline: Option<Underline>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modifiers: Option<HashSet<Modifier>>,
    }

    #[derive(Serialize, Clone)]
    pub struct Underline {
        pub color: Box<str>,
        pub style: UnderlineStyle,
    }

    #[derive(Serialize, PartialEq, Eq, Hash, Clone, Copy)]
    #[serde(rename_all = "snake_case")]
    #[allow(unused)]
    pub enum UnderlineStyle {
        Line,
        Curl,
        Dashed,
        Dotted,
        DoubleLine,
    }

    #[derive(Serialize, PartialEq, Eq, Hash, Clone, Copy)]
    #[serde(rename_all = "snake_case")]
    #[allow(unused)]
    pub enum Modifier {
        Bold,
        Dim,
        Italic,
        #[deprecated]
        Underlined,
        SlowBlink,
        RapidBlink,
        Reversed,
        Hidden,
        CrossedOut,
    }
}

mod token_color {
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TokenColor {
        #[serde(default)]
        pub settings: Settings,
        // None for scopeless global setting
        #[serde(default)]
        pub scope: Option<Scope>,
        // name is eliminated
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum Scope {
        Single(Box<str>),
        Multiple(Box<[Box<str>]>),
    }

    #[derive(Deserialize, Default, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Settings {
        #[serde(default)]
        pub foreground: Option<Box<str>>,
        #[serde(default)]
        pub font_style: FontStyle,
    }

    #[derive(Deserialize, Default, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    pub enum FontStyle {
        Italic,
        Bold,
        Strikethrough,
        Underline,

        #[default]
        None,
        #[serde(other)]
        Reset,
    }
}
