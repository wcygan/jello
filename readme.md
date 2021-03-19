# Jello
A simple word counting tool

### Installation via cargo

Install on any platform using Cargo:

```console
$ cargo install --git https://github.com/wcygan/jello
```

### How to run
`jello -l -w -c -f <file_1> <file_2> ... <file_n>`  
`... | jello` (piped)  
`jello < <file>` (redirected)

### Options

There are command line options that can be used:

| Option      | Usage                                                           | Example            |
| :---------- | :-------------------------------------------------------------- | :----------------- |
| `-f`        | Provides files to jello                                         | `-f abc.txt`       |
| `-l`        | Tells jello to show a line count                                | `-l`               |
| `-w`        | Tells jello to show a word count                                | `-w`               |
| `-c`        | Tells jello to show a character count                           | `-c `              |



### Libraries used
- [clap](https://docs.rs/clap/2.33.3/clap/) to parse commands  
- [rayon](https://docs.rs/rayon/1.5.0/rayon/) to parallelize execution