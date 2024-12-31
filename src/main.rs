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
    // 改行コードを統一
    let content = content.replace("\r\n", "\n").replace("\r", "\n");

    // マルチラインモードを有効にし、行末の改行もマッチさせるために \r?\n を追加
    let reference_pattern = r#"(?m)^\[(\d+)\]\s*(https?://\S+)(?:\n)?"#;
    let reference_regex = match Regex::new(reference_pattern) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("正規表現のコンパイルに失敗しました: {}", e);
            exit(1);
        }
    };

    // 番号 -> URL のマッピングを保存するハッシュマップ
    let mut references: HashMap<String, String> = HashMap::new();

    // すべてのマッチをイテレートしてマッピングを作成
    for caps in reference_regex.captures_iter(&content) {
        if let (Some(number), Some(url)) = (caps.get(1), caps.get(2)) {
            references.insert(number.as_str().to_string(), url.as_str().to_string());
        }
    }

    // デバッグ: 参照が正しく収集されたか確認
    // println!("収集された参照:");
    // for (number, url) in &references {
    //     println!("{} -> {}", number, url);
    // }

    // 参照行をメインコンテンツから削除（参照行とその改行を削除）
    let main_content = reference_regex.replace_all(&content, "").to_string();

    // メインコンテンツ内の [番号] をマッチする正規表現
    let ref_number_pattern = r#"\[(\d+)\]"#;
    let ref_number_regex = match Regex::new(ref_number_pattern) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("正規表現のコンパイルに失敗しました: {}", e);
            exit(1);
        }
    };

    // [番号] を [(番号)](URL) に置換
    let transformed_content = ref_number_regex.replace_all(&main_content, |caps: &regex::Captures| {
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

fn main() {
    // クリップボードから読み取る
    let clipboard_content = match read_from_clipboard() {
        Some(text) => text,
        None => {
            eprintln!("エラー: クリップボードに有効なテキストが含まれていません。");
            exit(1);
        }
    };

    // コンテンツを変換
    let converted_content = convert_markdown_references(&clipboard_content);
    // println!("変換されたコンテンツ:\n{}", converted_content);

    // クリップボードに書き戻す
    if write_to_clipboard(&converted_content) {
        println!("クリップボードの内容が変換されました。");
    } else {
        eprintln!("エラー: クリップボードに書き込めませんでした。");
        exit(1);
    }
}
