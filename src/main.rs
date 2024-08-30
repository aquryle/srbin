use std::fs::File;
use std::io::{self, BufRead, Write};

/// main関数
///
/// Sレコードファイルのデータレコードをバイナリファイルとして書き出す。
/// S0レコードはコンソールに文字列を表示する。
/// S1/S2/S3レコードはデータをバイナリに書き出す。
/// それ以外は無視する。
fn main() -> io::Result<()> {
    // S-Recordのパス
    let srec_path = "test/small_srec.mot";
    // Binaryのパス
    let bin_path = "test/output.bin";

    // S-Recordを開く
    let file = File::open(srec_path)?;
    let reader = io::BufReader::new(file);

    // Binaryを作って開く
    let mut bin_file = File::create(bin_path)?;

    // 1行ずつ読み出して処理する
    for line in reader.lines() {
        // 読み出し
        let line = line?;

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

            for i in (0..data.len()).step_by(2) {
                let byte = u8::from_str_radix(&data[i..i+2], 16).unwrap();
                bin_file.write_all(&[byte])?;
            }
        }
        // S5, S6, S7, S8, S9 は無視する
    }

    Ok(())
}



fn hex_to_string(hex: &str) -> String {
    let mut ascii_text = String::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i+2], 16).unwrap();
        ascii_text.push(byte as char);
    }
    ascii_text
}



