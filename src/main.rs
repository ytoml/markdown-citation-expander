use arboard::Clipboard;
use regex::Regex;
use std::collections::HashMap;
use std::process::exit;

/// クリップボードからテキストを読み取ります。
/// 成功した場合は `Some(String)`、失敗した場合は `None` を返します。
fn read_from_clipboard() -> Option<String> {
    let mut clipboard = Clipboard::new().ok()?;
    clipboard.get_text().ok()
}

/// 指定されたテキストをクリップボードに書き込みます。
/// 成功した場合は `true`、失敗した場合は `false` を返します。
fn write_to_clipboard(text: &str) -> bool {
    match Clipboard::new() {
        Ok(mut clipboard) => clipboard.set_text(text.to_owned()).is_ok(),
        Err(_) => false,
    }
}

/// 提供されたコンテンツ内のMarkdown参照を変換します。
/// `[番号] URL` の形式の行を `[(番号)](URL)` のMarkdownリンクに変換します。
fn convert_markdown_references(content: &str) -> String {
    let content = content.replace("\r\n", "\n").replace("\r", "\n");

    let reference_regex = Regex::new(r#"(?m)^\[(\d+)\]\s*(https?://\S+)(?:\n)?"#).unwrap();

    let mut references: HashMap<String, String> = HashMap::new();
    for caps in reference_regex.captures_iter(&content) {
        if let (Some(number), Some(url)) = (caps.get(1), caps.get(2)) {
            references.insert(number.as_str().to_string(), url.as_str().to_string());
        }
    }
    let main_content = reference_regex.replace_all(&content, "").to_string();

    let ref_number_regex = Regex::new(r#"\[(\d+)\]"#).unwrap();
    let transformed_content =
        ref_number_regex.replace_all(&main_content, |caps: &regex::Captures| {
            let number = &caps[1];
            if let Some(url) = references.get(number) {
                format!("[(記事{})]({}) ", number, url)
            } else {
                // 対応するURLが見つからない場合はそのままにする
                caps[0].to_string()
            }
        });

    transformed_content.to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clipboard_content = match read_from_clipboard() {
        Some(text) => text,
        None => {
            eprintln!("エラー: クリップボードに有効なテキストが含まれていません。");
            exit(1);
        }
    };
    let converted_content = convert_markdown_references(&clipboard_content);
    if write_to_clipboard(&converted_content) {
        println!("クリップボードの内容が変換されました。");
    } else {
        eprintln!("エラー: クリップボードに書き込めませんでした。");
        exit(1);
    }
    Ok(())
}
