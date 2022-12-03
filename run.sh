#!/bin/sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

pushd $SCRIPT_DIR > /dev/null
cargo build
popd > /dev/null

for bin in $SCRIPT_DIR/target/debug/day*
do
  if [[ $bin != *.d ]]
  then 
    day=$SCRIPT_DIR/data/`basename $bin`_data.txt
    cat $day | $bin
  fi
done
