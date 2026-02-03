use anyhow::{Result, Context};
use clap::Parser;
use ogg::{PacketWriter, PacketReader};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Parser)]
struct Args {
    /// 入力する .opus ファイルを複数指定
    inputs: Vec<String>,

    /// 出力ファイル
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.inputs.is_empty() {
        return Err(anyhow::anyhow!("入力ファイルが指定されていません"));
    }

    let out = File::create(&args.output)
        .with_context(|| format!("出力ファイルを作成できません: {}", args.output))?;
    let mut writer = PacketWriter::new(BufWriter::new(out));

    let mut granule_pos: u64 = 0;

    for path in args.inputs {
        let file = File::open(&path)
            .with_context(|| format!("入力ファイルを開けません: {}", path))?;
        let mut reader = PacketReader::new(BufReader::new(file));

        while let Some(packet) = reader.read_packet()? {
            let mut p = packet.data;

            // グラニュール位置を更新
            granule_pos += 960; // 20ms フレーム × 48kHz

            writer.write_packet(
                p,
                0,              // stream serial
                granule_pos,    // granule position
                false,          // last packet
            )?;
        }
    }

    Ok(())
}
