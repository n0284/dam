use clap::{Parser, Subcommand};

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
        Some(Commands::Get { name }) => {
            println!("{}ダムの貯水率を取得します…", name);
            // TODO: nameからIDを引いてスクレイピング
        }
        Some(Commands::All) => {
            println!("9ダム合計の貯水率を取得します…");
            // TODO: 9ダム分のスクレイピング
        }
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
}
