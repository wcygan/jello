# Jello
A simple word counting tool

### How to run
`jello -l -w -c -f <file_1> <file_2> ... <file_n>`  
`... | jello` (piped)  
`jello < <file>` (redirected)

### Modifiers
`-f`: supplies one to many files to inspect  
`-l`: counts the lines in files   
`-w`: counts the words in files  
`-c`: counts the characters in files  

### Libraries used
- [clap](https://docs.rs/clap/2.33.3/clap/) to parse commands  
- [rayon](https://docs.rs/rayon/1.5.0/rayon/) to parallelize execution