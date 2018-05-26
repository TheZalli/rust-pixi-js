#!/bin/sh
if [ "$1" = "all" ]; then
  cargo clean
  rm -rf wasm_bindgen/
  rm -rf node_modules/
elif [ "$1" = "help" ] || [ "$#" = "0" ]; then
  echo "./clean.sh all | ([ cargo | wbind | node ]*)\n"
  echo "Cleans the chosen build files."
else
  for var in "$@"
  do
    if [ ${var} = "cargo" ]; then
      cargo clean
    elif [ ${var} = "wbind" ]; then
      rm -rf wasm_bindgen/
    elif [ ${var} = "node" ]; then
      rm -rf node_modules/
    fi
  done
fi
