use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// ダム名を指定して貯水率を取得
    Get {
        /// ダムの名前（例: 矢木沢）
        name: String,
    },
    /// 9ダム合計の貯水率を取得
    All,
    /// 対応しているダム一覧を表示
    List,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();

    match &cli.command {
        // 指定したダムの貯水率
        Some(Commands::Get { name }) => {
            let dam_ids = get_dam_id_map();
            if let Some(id) = dam_ids.get(name.as_str()) {
                match fetch_storage_rate(id) {
                    Ok(rate) => println!("{}ダムの貯水率は {} です", name, rate),
                    Err(e) => eprintln!("取得エラー: {}", e),
                }
            } else {
                eprintln!("対応していないダム名です: {}", name);
            }
        }
        // 9ダム合計
        Some(Commands::All) => {
            let allrate = fetch_all_dam_rate()?;
            println!("9ダム合計の貯水率（前日0時時点）: {}%", allrate);
        }
        // 対応ダム一覧
        Some(Commands::List) => {
            println!("対応ダム一覧:");
            println!("・矢木沢");
            println!("・奈良俣");
            println!("・藤原");
            // etc.
        }
        None => {
            println!("コマンドを指定してください。例: dam get 矢木沢");
        }
    }

    // ダム名 -> 観測所番号 の対応表を返す関数
    fn get_dam_id_map() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("矢木沢", "1368030375010"),
            ("奈良俣", "1368030375020"),
            ("藤原", "1368030375030"),
            ("相俣", "1368030375090"),
            ("薗原", "1368030375130"),
            ("下久保", "1368030375210"),
            ("草木", "1368030375180"),
            // 八ッ場、渡良瀬貯水池は別対応なので省略中
        ])
    }

    // 指定されたダムIDから貯水率を取得する
    fn fetch_storage_rate(id: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 1. HTMLページ取得
        let url = format!(
            "https://www1.river.go.jp/cgi-bin/DspDamData.exe?ID={}&KIND=3&PAGE=0",
            id
        );
        let client = Client::builder().build()?;
        let res = client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .send()?;
        if !res.status().is_success() {
            return Err(format!("HTTPエラー: {}", res.status()).into());
        }
        let html = res.text()?;

        // 2. .datのhrefを抽出
        let document = Html::parse_document(&html.to_lowercase());
        let selector = Selector::parse("a").unwrap();

        let dat_href = document
            .select(&selector)
            .filter_map(|e| e.value().attr("href"))
            .find(|href| href.ends_with(".dat"))
            .ok_or("datファイルへのリンクが見つかりませんでした")?;

        // 3. 完全URLに変換
        let dat_url = format!("https://www1.river.go.jp{}", dat_href);
        println!("datのURL→{}", dat_url);

        // 4. datファイルの中身を取得
        let dat_text = get(&dat_url)?.text()?;

        // 5. 一番下の行（最新データ）をパース
        let last_line = dat_text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .last()
            .ok_or("datファイルが空です")?;

        // 6. カンマ区切りで分割 → 3列目あたりが貯水率
        let fields: Vec<&str> = last_line.split(',').collect();
        let rate = fields.get(10).ok_or("貯水率の列が見つかりません")?;

        Ok(rate.to_string())
    }

    // 9ダム合計
    fn fetch_all_dam_rate() -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://www.waterworks.metro.tokyo.lg.jp/suigen/suigen";
        let client = Client::builder().build()?;

        let res = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0") // 念のため
            .send()?;

        let body = res.text()?;
        let document = Html::parse_document(&body);

        // <tr> 全体を探索
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        for tr in document.select(&tr_selector) {
            let tds: Vec<_> = tr.select(&td_selector).collect();
            if tds.len() >= 5 {
                let header = tds[0].text().collect::<String>().trim().to_string();
                if header.contains("以上合計") {
                    // index 4 が 5番目（0-based）
                    let rate = tds[5].text().collect::<String>().trim().to_string();
                    return Ok(rate);
                }
            }
        }

        Err("9ダム合計の貯水率が見つかりませんでした".into())
    }
    Ok(())
}
