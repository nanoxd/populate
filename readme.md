# Populate

A population analyzer to be used with the [world population dataset][data]

## Installation

```sh
cargo install --git https://github.com/nanoxd/populate.rs.git
```

## Usage

#### From STDIN
```sh
cat worldcitiespop.csv | populate CITY
```

#### Specifying a file

```sh
populate -f FILE CITY
```

[data]: http://burntsushi.net/stuff/worldcitiespop.csv.gz
