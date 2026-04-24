mod codegen;
mod ir;

use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(version, about = "Generates LaTeX from kursach IR yaml")]
struct Cli {
    input: PathBuf,

    #[arg(short, long, default_value = "out")]
    output: PathBuf,

    #[arg(long)]
    no_compile: bool,

    #[arg(long, default_value = "xelatex")]
    xelatex: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let doc = ir::load(&cli.input)?;
    let latex = codegen::generate(&doc)?;

    std::fs::create_dir_all(&cli.output)?;
    let tex_path = cli.output.join("main.tex");
    std::fs::write(&tex_path, &latex)?;
    println!("Written: {}", tex_path.display());

    if !cli.no_compile {
        run_xelatex(&cli.xelatex, &tex_path, &cli.output)?;
        run_xelatex(&cli.xelatex, &tex_path, &cli.output)?;
        let pdf = cli.output.join("main.pdf");
        println!("Done: {}", pdf.display());
    }

    Ok(())
}

fn run_xelatex(xelatex: &PathBuf, tex_path: &PathBuf, output_dir: &PathBuf) -> Result<()> {
    println!("Running: {} {}", xelatex.display(), tex_path.display());

    let status = Command::new(xelatex)
        .arg("-interaction=nonstopmode")
        .arg("-output-directory")
        .arg(output_dir)
        .arg(tex_path)
        // .stdout(std::process::Stdio::null())
        // .stderr(std::process::Stdio::null())
        .status()
        .with_context(|| format!(
            "Failed to launch xelatex ('{}'); is it installed and in PATH?",
            xelatex.display()
        ))?;

    if !status.success() {
        let log = output_dir.join("main.log");
        bail!(
            "xelatex exited with {}. Check the log: {}",
            status,
            log.display()
        );
    }

    Ok(())
}