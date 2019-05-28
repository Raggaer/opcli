# opcli

Basic wrapper for the 1Password `op` command line application.
Basically its just a simple wrapper to do `op` actions (view, edit, delete an item, ...)

You can use the flag `-h` to get more help about all the possible options.

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
