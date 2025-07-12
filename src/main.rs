use clap::{Parser, Subcommand};
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
fn main() {
    let cli = Args::parse();

    match &cli.command {
        // 指定したダムの貯水率
        Some(Commands::Get { name }) => {
            let dam_ids = get_dam_id_map();
            if let Some(id) = dam_ids.get(name.as_str()) {
                println!("{}ダムのIDは {} です", name, id);
                // この ID を使って URL を作ってスクレイピングへ進む
            } else {
                eprintln!("対応していないダム名です: {}", name);
            }
        }
        // 9ダム合計
        Some(Commands::All) => {
            println!("9ダム合計の貯水率を取得します…");
            // TODO: 9ダム分のスクレイピング
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
}
