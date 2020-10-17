# SIFA
A simple Nix script template generator

I keep on forgetting how to write simple nix files.

So here is a simple template generator with the possibility of loading some predefined config.

## Install

```bash
nix-env -i -f https://github.com/GuilloteauQ/SIFA/tarball/main
```

## Utilisation

`sifa` only produces output to `stdout`.

This way we can use it in different ways

### One time `nix-shell`

We can use it to quickly enter a shell.

In this example we load our `python` profile.

```bash
nix-shell -E "$(sifa python)"
```

### Long time `nix-shell`

For shells that you will come back to, you can simply redirect the output of `sifa` in a file

```bash
sifa python > shell.nix
nix-shell
```

## Profiles

We call profiles a predefined configuration/template for `sifa`.

We store those in a `json` file as follows:

```json
# ~/.sifa.json

{
    "R" :{
        "name": "Rggplot",
        "packages": ["R", "rPackages.ggplot2"]
    },
    "python" :{
        "name": "python",
        "packages": ["python"]
    }
}
```

In this example, we have 2 profiles (`R` and `python`).

So when we call `sifa` with one of the profiles name, it will generate a nix file with the proper configuration.
