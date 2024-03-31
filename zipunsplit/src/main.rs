// SPDX-FileCopyrightText: 2024 Andrew Gunnerson
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::File,
    io::{self, Seek},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;

use zipunsplitlib::file::{JoinedFile, MemoryCowFile, Opener};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    /// Split input files.
    ///
    /// The files must be specified in order.
    #[arg(value_parser, value_name = "FILE")]
    pub input: Vec<PathBuf>,

    /// Joined output file.
    #[arg(short, long, value_parser, value_name = "FILE")]
    pub output: PathBuf,
}

struct FixedOpener<'a, P: AsRef<Path>> {
    paths: &'a [P],
}

impl<'a, P: AsRef<Path>> Opener for FixedOpener<'a, P> {
    fn open_split(&mut self, index: usize) -> io::Result<File> {
        File::open(&self.paths[index])
    }

    fn num_splits(&self) -> usize {
        self.paths.len()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let joined = JoinedFile::new(FixedOpener { paths: &cli.input })?;

    let split_ranges = joined.splits();
    let mut cow_file = MemoryCowFile::new(joined, 4096)?;
    zipunsplitlib::split::fix_offsets(&mut cow_file, &split_ranges)
        .context("Failed to fix split zip offsets")?;
    cow_file.rewind()?;

    let mut file = File::create(&cli.output)
        .with_context(|| format!("Failed to create file: {:?}", cli.output))?;

    io::copy(&mut cow_file, &mut file)
        .context("Failed to copy data from split files to output file")?;

    Ok(())
}
