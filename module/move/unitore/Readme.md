
# Module :: unitore
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTemplateBlankPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTemplateBlankPush.yml) [![docs.rs](https://img.shields.io/docsrs/willbe?color=e3e8f0&logo=docs.rs)](https://docs.rs/unitore) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Feed reader with the ability to set updates frequency.

### Basic use-case

To start using unitore, create configuration toml file with list of feed information - its link and update period.
Example:


```toml
[[config]]
name = "bbc" 
period = "2days" 
link = "https://feeds.bbci.co.uk/news/world/rss.xml"

[[config]]
name = "times"
period = "2days"
link = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"

```
Add created config file to unitore storage using command `.config.add` with path to config file.
You can add more than one file, by executing `.config.add` for every file. Example:
```bash
cargo run .config.add ./config/feeds.toml
```
To download feeds from sources specified in config file into storage use command `.frames.download`.
Every time this command is run, feeds from all sources listed in all config files will be updated.
```bash
cargo run .frames.download
```
To get all frames that are currently in storage run:
```bash
cargo run .frames.list
```
To get all feeds that are currently in storage run:
```bash
cargo run .feeds.list
```
To get custom information about feeds or frames run SQL query to storage database using command `.query.execute` with query string:
```bash
cargo run .query.execute \'SELECT title, links, MIN\(published\) FROM Frames\'
```
To remove config file from storage use command `.config.delete` with path to config file:
```bash
cargo run .config.delete ./config/feeds.toml
```
To see all config files with feed sources:
```bash
cargo run .config.list
```


### To add to your project

```bash
cargo add unitore
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --package unitore
```
