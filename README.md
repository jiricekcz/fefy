# fefy
CLI for simple usage of the FEF file format

## Usage

Fefy is a simple CLI tool that will allow you to make and evaluate FEF files.

### Evaluating FEF files

Fefy can evaluate Single Formula FEF file, where all variables used are named.

Fefy evaluates the formula in binary 64-bit floating point arithmetic with the standard interpretation of FEF expressions.

To evaluate a FEF file, use the `evaluate` subcommand.

```bash
fefy evaluate --input <file>
```

You will be prompted for the values of the variables used in the formula.

### Creating FEF files

Fefy can create FEF files from a simple expression written in a human readable infix format.

To create a FEF file, use the `create` subcommand.

```bash
fefy create --output <file>
```

You will be prompted for the expression of the formula and formula name. If you leave the name blank, no name will be used.

To create a FEF file from a text formula file, use the `create` subcommand with the `--input` flag.

```bash
fefy create --output <file> --input <file>
```

You will only be prompted for a name.

#### Expression language

Use the following operators in the expression:
- `+` for addition or identity
- `-` for subtraction or negation
- `*` for multiplication
- `/` for division
- `%` for modulo
- `//` for integer division
- `^` or `**` for exponentiation
- `()` for grouping

Sequences of letters and similar characters are considered variable names.

Note, that other operators may be defined to prevent confusion with variable names. They will however always cause an illegal operator use error. 

## Building from source

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads) (only for cloning the repository)

Clone the repository 
```bash
git clone https://github.com/jiricekcz/fefy
```

Navigate to the repository
```bash
cd fefy
```

Build the project with optimizations
```bash
cargo build --release
```
Note, that this will require an internet connection to download the dependencies.

The binary will be located in `target/release/fefy`