# DemocracyCraft Budget Transcript to CSV

<a href="https://bsky.app/profile/borealsoul.space"><img src="https://img.shields.io/badge/Bluesky-0285FF?logo=bluesky&logoColor=fff&style=for-the-badge" /></a>
![image](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)
![image](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)
![image](https://img.shields.io/badge/Zed-white?style=for-the-badge&logo=zedindustries&logoColor=084CCF)

A Rust tool to convert DC's budget transcripts into a delimited by semicolons CSV.

## Installation
Check the [release](https://github.com/borealsoul/DC-Budget-to-CSV/releases) page, or build it from the source!

```
cd budget-to-csv
cargo build -p budget-to-csv --release
mv ./target/release/budget-to-csv "your prefered path"
```

## Usage
```
Usage: budget-to-csv <input.txt> <output.csv>
```
