extern crate regex;

use regex::Regex;
// use flate2::read::GzDecoder;
// use chrono::Local;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    const ABSTRUCT_FILE_PATH: &str = "./jawiki-latest-abstract.xml";
    // const ABSTRUCT_FILE_PATH: &str = "./jawiki-latest-abstract.xml.gz";
    let file = File::open(ABSTRUCT_FILE_PATH)?;
    let file = BufReader::new(file);
    // let gz = GzDecoder::new(file);
    // gz.header().expect("Invalid gz header");

    // for (index, line) in io::BufReader::new(gz).lines().enumerate() {
    //     println!(">>{}, {:?}", index, line);
    // }

    //     // MS-IMEヘッダ出力
    //     println!(
    //         r#"!Microsoft IME Dictionary Tool
    // !Version:
    // !Format:WORDLIST
    // !wikipedia-ime-dictionary
    // !Output File Name:
    // !DateTime:{}"#,
    //         Local::now()
    //     );

    // xml読み込み（GZIP展開がうまくいかないので仮）
    let mut title = String::new();
    for line in &mut file.lines() {
        let line = line.unwrap();
        // <title><abstruct>から項目名と読み仮名を抽出
        if line.starts_with("<title>") {
            let regex_title = Regex::new(r"<title>Wikipedia: (.+)</title>").unwrap();
            match regex_title.captures(&line) {
                Some(v) => {
                    title = v.at(1).unwrap().to_string();
                }
                None => {}
            }
        }
        if line.starts_with("<abstract>") {
            let regex_abstract = Regex::new(r"<abstract>(.+)</abstract>").unwrap();
            if let Some(abstr) = regex_abstract.captures(&line) {
                let abstr = abstr.at(1).unwrap_or("");
                // <abstruct>内の"項目名（読み仮名）"となっている部分を採用
                if let Ok(regex_yomi) =
                    Regex::new(&(title.to_string() + "（([\u{3041}-\u{3096}]+)）"))
                {
                    if let Some(yomi) = regex_yomi.captures(&abstr) {
                        let yomi = yomi.at(1).unwrap_or("");
                        //よみ\t単語\t品詞\tユーザコメント
                        println!(
                            "{}\t{}\t{}\t{}",
                            yomi,
                            title,
                            "名詞",
                            abstr.lines().next().unwrap_or("")
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
