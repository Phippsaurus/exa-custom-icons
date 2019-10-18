use ansi_term::Style;
use fs::File;
use info::filetype::FileExtensions;
use output::file_name::FileStyle;

pub trait FileIcon {
    fn icon_file(&self, file: &File) -> Option<char>;
}

pub enum Icons {
    Archive,
    Audio,
    Binary,
    Crypto,
    Image,
    Temp,
    Video,
}

impl Icons {
    pub fn value(&self) -> char {
        match *self {
            Icons::Archive => '\u{f1c6}',
            Icons::Audio => '\u{f001}',
            Icons::Binary => '\u{f471}',
            Icons::Crypto => '\u{e60a}',
            Icons::Image => '\u{f1c5}',
            Icons::Temp => '\u{f56a}',
            Icons::Video => '\u{f03d}',
        }
    }
}

pub fn painted_icon(file: &File, style: &FileStyle) -> String {
    let file_icon = icon(&file).to_string();
    let painted = style
        .exts
        .colour_file(&file)
        .map_or(file_icon.to_string(), |c| {
            // Remove underline from icon
            if c.is_underline {
                match c.foreground {
                    Some(color) => Style::from(color).paint(file_icon).to_string(),
                    None => Style::default().paint(file_icon).to_string(),
                }
            } else {
                c.paint(file_icon).to_string()
            }
        });
    format!("{} ", painted)
}

fn icon(file: &File) -> char {
    let extensions = Box::new(FileExtensions);
    if file.is_directory() {
        '\u{f07c}'
    } else if let Some(icon) = extensions.icon_file(file) {
        icon
    } else if let Some(ext) = file.ext.as_ref() {
        match ext.as_str() {
            "ai" => '\u{e7b4}',
            "android" => '\u{e70e}',
            "apple" => '\u{f179}',
            "avro" => '\u{e60b}',
            "c" => '\u{e61e}',
            "clj" => '\u{e768}',
            "coffee" => '\u{f0f4}',
            "conf" | "toml" | "ini" => '\u{e615}',
            "cmake" => '\u{e20f}',
            "cpp" | "cc" | "cxx" => '\u{e61d}',
            "cs" | "csx" => '\u{f81a}',
            "css" => '\u{e749}',
            "d" => '\u{e7af}',
            "dart" => '\u{e798}',
            "db" | "sql" | "dump" => '\u{f1c0}',
            "desktop" => '\u{f108}',
            "diff" => '\u{f440}',
            "doc" => '\u{f1c2}',
            "ebook" => '\u{e28b}',
            "env" => '\u{f462}',
            "epub" => '\u{e28a}',
            "erl" => '\u{e7b1}',
            "ex" | "exs" => '\u{e62d}',
            "fs" | "fsx" => '\u{e7a7}',
            "font" => '\u{f031}',
            "gform" => '\u{f298}',
            "git" => '\u{f1d3}',
            "go" => '\u{e626}',
            "h" | "hpp" | "hh" | "hxx" => '\u{f0fd}',
            "hs" | "cabal" => '\u{e777}',
            "htm" | "html" => '\u{f13b}',
            "iml" => '\u{e7b5}',
            "java" => '\u{e204}',
            "js" => '\u{e74e}',
            "json" => '\u{e60b}',
            "jsx" => '\u{e7ba}',
            "less" => '\u{e758}',
            "lib" | "rlib" | "so" => '\u{f02d}',
            "lock" => '\u{f456}',
            "log" => '\u{f18d}',
            "lua" => '\u{e620}',
            "md" | "markdown" | "mdx" | "rmd" => '\u{e609}',
            "mustache" | "hbs" => '\u{e60f}',
            "npmignore" => '\u{e71e}',
            "pdf" => '\u{f1c1}',
            "php" => '\u{e73d}',
            "pl" => '\u{e769}',
            "ppt" => '\u{f1c4}',
            "psd" => '\u{e7b8}',
            "py" | "pyc" | "pyo" | "pyd" => '\u{e606}',
            "r" => '\u{f25d}',
            "rb" => '\u{e21e}',
            "rdb" => '\u{e76d}',
            "rs" => '\u{e7a8}',
            "rss" => '\u{f09e}',
            "rubydoc" => '\u{e73b}',
            "sass" | "scss" => '\u{e603}',
            "scala" => '\u{e737}',
            "shell" | "sh" | "bat" | "bash" | "bashrc" | "zsh" => '\u{f489}',
            "sln" | "suo" => '\u{fb0f}',
            "sqlite3" => '\u{e7c4}',
            "styl" => '\u{e600}',
            "svg" => '\u{fc1f}',
            "swift" => '\u{fbe3}',
            "tex" => '\u{e600}',
            "ttf" | "otf" => '\u{fbd4}',
            "ts" | "tsx" => '\u{e628}',
            "twig" => '\u{e61c}',
            "txt" => '\u{f15c}',
            "video" => '\u{f03d}',
            "vim" | "vimrc" | "nvim" => '\u{e62b}',
            "xls" => '\u{f1c3}',
            "xml" => '\u{fabf}',
            "yml" | "yaml" => '\u{f481}',
            _ => '\u{f15b}',
        }
    } else {
        match file.name.as_str() {
            "LICENSE" | "LICENCE" | "license" => '\u{f1f9}',
            "Dockerfile" | "dockerfile" => '\u{f308}',
            "Godeps" => '\u{e626}',
            "Makefile" => '\u{e20f}',
            "vimrc" | ".vimrc" => '\u{e7c5}',
            "Vagrantfile" => '\u{f2b8}',
            ".zshrc" | ".bashrc" | "terminalrc" => '\u{e795}',
            ".gitconfig" | ".gitignore" | ".gitmodules" => '\u{e702}',
            _ => '\u{f15b}',
        }
    }
}
