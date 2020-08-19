use chrono::Local;
use guid_create::GUID;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

// MS-IMEオープン辞書(dctx)ファイルオープン
fn open_dictionary(dict_count: i64) -> std::io::BufWriter<std::fs::File> {
    let path = &format!("Wikipedia日本語辞書{:04}.dctx", dict_count);

    println!("ファイルオープン : {}", path);
    BufWriter::new(File::create(path).expect(&format!("ファイルオープン失敗({})", path)))
}

// MS-IMEオープン辞書(dctx)ヘッダ出力
fn output_header(output_file: &mut std::io::BufWriter<std::fs::File>, name: &str) {
    let header: String = format!(
        r#"<ns1:Dictionary xmlns:ns1="http://www.microsoft.com/ime/dctx" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<ns1:DictionaryHeader>
<ns1:DictionaryGUID>{{{}}}</ns1:DictionaryGUID>
<ns1:DictionaryLanguage>ja-jp</ns1:DictionaryLanguage>
<ns1:DictionaryVersion>{}</ns1:DictionaryVersion>
<ns1:SourceURL>http://download.wikimedia.org/jawiki/</ns1:SourceURL>
<ns1:CommentInsertion>true</ns1:CommentInsertion>
<ns1:DictionaryInfo Language="ja-jp">
    <ns1:ShortName>{}</ns1:ShortName>
    <ns1:LongName>{}</ns1:LongName>
    <ns1:Description>Wikipediaから生成した辞書です。生成日時{}</ns1:Description>
    <ns1:Copyright>この辞書はウィキペディア(http://ja.wikipedia.org/)のテキストを利用しています。テキストはクリエイティブ・コモンズ表示-継承ライセンス(CC-BY-SA)の下で利用可能です。追加の条件が適用される場合があります。詳細はウィキペディアの利用規約を参照してください。ライセンスのURL： http://creativecommons.org/licenses/by-sa/3.0/deed.ja </ns1:Copyright>
    <ns1:CommentHeader1></ns1:CommentHeader1>
</ns1:DictionaryInfo>
</ns1:DictionaryHeader>
"#,
        GUID::rand(), // DictionaryGUID
        1,            // DictionaryVersion
        name,         // ShortName
        name,         // LongName
        Local::now()  //CommentHeader1
    );

    output_file
        .write(header.as_bytes())
        .expect("ヘッダのファイル出力失敗");
}

// MS-IMEオープン辞書(dctx)エントリ出力
fn output_entry(
    output_file: &mut std::io::BufWriter<std::fs::File>,
    input_string: &str,
    output_string: &str,
    comment_data: &str,
    url: &str,
) {
    let entry: String = format!(
        r#"
<ns1:DictionaryEntry>
<ns1:InputString>{}</ns1:InputString>
<ns1:OutputString>{}</ns1:OutputString>
<ns1:PartOfSpeech>Noun</ns1:PartOfSpeech>
<ns1:CommentData1>{}</ns1:CommentData1>
<ns1:URL>{}</ns1:URL>
<ns1:Priority>200</ns1:Priority>
<ns1:ReverseConversion>true</ns1:ReverseConversion>
<ns1:CommonWord>false</ns1:CommonWord>
</ns1:DictionaryEntry>
"#,
        input_string, output_string, comment_data, url
    );
    output_file
        .write(entry.as_bytes())
        .expect("エントリのファイル出力失敗");
}

// MS-IMEオープン辞書(dctx)フッタ出力
fn output_footer(output_file: &mut std::io::BufWriter<std::fs::File>) {
    let footer: String = "</ns1:Dictionary>".to_string();

    output_file
        .write(footer.as_bytes())
        .expect("フッタのファイル出力失敗");
}

fn main() -> Result<(), std::io::Error> {
    const ABSTRUCT_FILE_PATH: &str = "./jawiki-latest-abstract.xml";

    // xml読み込み（GZIP展開がうまくいかないので仮）
    let input_file = BufReader::new(File::open(ABSTRUCT_FILE_PATH).unwrap());
    let mut dict_count = 0;
    let mut word_count = 0;
    let mut output_file = open_dictionary(dict_count);
    let mut output_string = String::new();
    let mut url = String::new();
    let regex_title = Regex::new(r"<title>Wikipedia: (.+)</title>").unwrap();
    let regex_url = Regex::new(r"<url>(.+)</url>").unwrap();
    let regex_abstr = Regex::new(r"<abstract>(.+)</abstract>").unwrap();
    for line in &mut input_file.lines() {
        let line = line.unwrap();
        // <title><url><abstruct>から項目名、URL、読み仮名を抽出
        if line.starts_with("<title>") {
            output_string = regex_title
                .captures(&line)
                .map(|v| v.at(1).unwrap().to_string())
                .unwrap_or(String::new());
        } else if line.starts_with("<url>") {
            url = regex_url
                .captures(&line)
                .map(|v| v.at(1).unwrap().to_string())
                .unwrap_or(String::new());
        } else if line.starts_with("<abstract>") {
            if let Some(abstr) = regex_abstr.captures(&line) {
                let abstr = abstr.at(1).unwrap_or("");
                // <abstruct>内の"項目名（読み仮名）"となっている部分を採用
                if let Ok(regex_input_string) =
                    Regex::new(&(output_string.to_string() + "（([\u{3041}-\u{3096}]+)）"))
                {
                    if let Some(input_string) = regex_input_string.captures(&abstr) {
                        let input_string = input_string.at(1).unwrap_or("");

                        // dctxには36万語制限あり。仮で20万語に制限する
                        if word_count % 200000 == 0 {
                            output_footer(&mut output_file);
                            output_file = open_dictionary(dict_count);
                            output_header(
                                &mut output_file,
                                &format!("Wikipedia日本語辞書{:04}", dict_count),
                            );
                            dict_count += 1;
                            word_count = 0;
                        }
                        output_entry(&mut output_file, input_string, &output_string, abstr, &url);
                        if word_count % 100 == 0 {
                            print!("単語数 : {:08}\r", word_count);
                            //std::io::stdout().flush()?;
                        }
                        word_count += 1;
                    }
                }
            }
        }
    }
    output_footer(&mut output_file);

    Ok(())
}
