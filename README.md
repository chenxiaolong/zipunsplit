# zipunsplit

zipunsplit is a tiny tool for joining split zip files, as defined in [section 8.2 of the specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT). These are zip files that make use of the disk number fields in the headers.

It works by combining all of the split files and updating the disk-relative offset fields to absolute offsets. Most well-formed split zips, including those that use zip64, are supported. The exceptions are:

* Zip files with encrypted central directory entries are not supported.
* Zip files with an archive comment field that contains the EOCD magic bytes (`PK\x05\x06`) are not supported.
* Zip files that were split naively (i.e. rejoinable by simple concatenation) are not supported.

## Usage

```
zipunsplit -o <output> [<input> ...]
```

For example:

```
zipunsplit -o output.zip input.z01 input.z02 input.zip
```

The inputs must be specified in the proper order. Usually, the files are named with the `.z<number>` extension, with the `.zip` being the last file.

The output will be a well-formed regular unsplit zip file. It is guaranteed that writing to the output file is done sequentially. Writing to a pipe is supported.

## Building from source

First, make sure the [Rust toolchain](https://rust-lang.org/) is installed. Then, run:

```
cargo build --release
```

The resulting executable will be in `target/release/zipunsplit` or `target\release\zipunsplit.exe`.

## Use as a library

The logic is all implemented in the `zipunsplitlib` library in this repo.

## Contributing

([AI policy](https://github.com/chenxiaolong/chenxiaolong/blob/master/AI_POLICY.md))

Bug fix pull requests are welcome and much appreciated!

If you are interested in implementing a new feature and would like to see it included in zipunsplit, please open an issue to discuss it first.

## License

zipunsplit is licensed under GPL-3.0-only. Please see [`LICENSE`](./LICENSE) for the full license text.
