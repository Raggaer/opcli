# opcli

Basic wrapper for the 1Password `op` command line application, mostly for items.
Basically its just a simple wrapper to do `op` actions (view, edit, delete an item, ...)

You can use the flag `-h` to get more help about all the possible options.

# Authentication

This application does not handle authentication, you should use `op signin` before using `opcli`.
Its recommended to signin everytime you want to retrieve a password. You can do the `eval ...` trick and keep the terminal session ope,
that way you can use `opcli` before your session ends.

## Get an item

In order to retrieve an item, you need to use the following flags:

- `-c`: Specify the command we want to run, in this case `get`.
- `-s`: The type of object we want to retrieve, in this case `item`.
- `-i`: The name or UUID of the item you want to retrieve.

There are some optional flags you can use, to retrieve item information and write it to your terminal
or to copy the item password directly to your clipboard (using `xclip`).

- `-f`: Comma separated list of the fields you want to retrieve (username, uuid, password, ...).
- `--pw`: Copy the item password directly to your clipboard. Handy for logins.


```
./opcli -c "get" -s "item" -i "My Super Secure Email Login" -f "username, password"
```

## Search for an item

In order to search for an item (for example you dont remember the exact name, or want to search for its UUID), 
you need to use the following flags:

- `-c`: Specify the command we want to run, in this case `list`.
- `-s`: The type of object we want to retrieve, in this case `item`.
- `--search` Substring of the item name or URL you want to look for.

```
./opcli -c "list" -s "item" --search "uni website"
```

# License

`opcli` is licensed under the MIT license.
