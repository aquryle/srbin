use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::env;
use std::path::Path;


/// main関数
///
/// Sレコードファイルのデータレコードをバイナリファイルとして書き出す。
/// S0レコードはコンソールに文字列を表示する。
/// S1/S2/S3レコードはデータをバイナリに書き出す。
/// それ以外は無視する。
fn main() {

    // 引数のチェック
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("使用方法：srbin file/path/to/srec");
        std::process::exit(1);
    }

    // ファイルパス設定
    let input_path = Path::new(&args[1]);
    let output_path = input_path.with_extension("bin");

    // 出力ファイルの削除確認
    if output_path.exists() {
        println!("{}は既に存在します。削除してもよろしいですか？(y/n)", output_path.display());
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("入力を読み取れませんでした");
        if response.trim().to_lowercase() != "y" {
            println!("プログラムを終了します");
            std::process::exit(0);
        }
    }

    // ファイルオープン
    let mut output_file = File::create(&output_path).expect("出力ファイルを作成できませんでした");
    let file = File::open(input_path).expect("入力ファイルを開けませんでした");
    let reader = BufReader::new(file);

    // 1行ずつ読み出して処理する
    for line in reader.lines() {
        // 読み出し
        let line = line.expect("行を読み出せません");


        // レコードタイプで分岐する
        if line.starts_with("S0") {
            // S0はコンソールへ表示する
            let text_hex = &line[8..line.len() - 2];  // データ部をそのまま文字列に格納する
            let text = hex_to_string(text_hex);     // データ部文字列をテキスト変換して格納する
            println!("Record Label: {}", text);             // 表示する
        } else if line.starts_with("S1") || line.starts_with("S2") || line.starts_with("S3") {
            // S1/S2/S3はデータを取り出してバイナリファイルに書き出す
            let count = usize::from_str_radix(&line[2..4], 16).unwrap(); // レコード長
            let data_start = match line.chars().nth(1).unwrap() {
                // レコードタイプごとのデータレコード開始位置を設定する
                '1' => 8,   // レコードタイプ(2) + データ長(2) + アドレス(4)
                '2' => 10,  // レコードタイプ(2) + データ長(2) + アドレス(6)
                '3' => 12,  // レコードタイプ(2) + データ長(2) + アドレス(8)
                _ => panic!("Format Error!"),
            };  // データレコード開始位置
            let data_end = 2 + 2 + 2 * count - 2;    // データレコード終了位置（レコードタイプ + レコード長 + アドレス以降 - SUM）
            let data = &line[data_start..data_end];  // データレコード最初〜最後の文字列

            // 2文字ずつ取り出してバイナリ変換し、ファイルに書く
            for i in (0..data.len()).step_by(2) {
                let byte = u8::from_str_radix(&data[i..i+2], 16).unwrap();
                output_file.write_all(&[byte]).expect("バイナリデータを読み出せません");
            }
        }
        // S5, S6, S7, S8, S9 は無視する
    }
}



fn hex_to_string(hex: &str) -> String {
    let mut ascii_text = String::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i+2], 16).unwrap();
        ascii_text.push(byte as char);
    }
    ascii_text
}



