> “The whole of life is just like watching a click, he thought. Only it’s as though you always get in ten minutes after the big picture has started, and no-one will tell you the plot, so you have to work it all out yourself from the clues.”
> 
> ― Terry Pratchett, Moving Pictures

# Twine

Directly fetch gifs from [getyarn.io](https://getyarn.io)

## Installation

```bash
$ cargo install twine-gif
```

## Usage

```bash
Usage: twine [OPTIONS] <URL>

Arguments:
  <URL>  A valid yarn URL

Options:
  -o, --output <OUTPUT>  A filename to which the gif will be written
  -h, --help             Print help

```

```bash
$ twine 'https://getyarn.io/yarn-clip/b3d2898d-ed2a-4d71-a3a3-88b8355e04a1' > redirect.gif ## redirect to a file

$ twine 'https://getyarn.io/yarn-clip/b3d2898d-ed2a-4d71-a3a3-88b8355e04a1' -o yourfile.gif

```

Made with <3 and rust by [kingsfoil](https://kingsfoil.surge.sh/about.html)