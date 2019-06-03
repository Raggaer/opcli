#!/bin/bash

# Function to retrieve 1Password items using opcli
function pwget {
	if [ -n "$1" ]; then
		if [ -z "${OP_SESSION_my}" ]; then
			eval $(op signin)
		fi
		opcli -c "get" -s "item" -i "$1" --pw
	else
		echo "Provide an item to retrieve the password!"
	fi
}

# Function to search for 1Password items
function pwsearch {
	if [ -n "$1" ]; then
		if [ -z "${OP_SESSION_my}" ]; then
			eval $(op signin)
		fi
		opcli -c "list" -s "items" --search "$1"
	else
		echo "Provide a name to search for!"
	fi
}
