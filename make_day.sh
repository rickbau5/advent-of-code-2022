#!/usr/bin/env bash

set -e

[ -z $1 ] && { echo "first argument required"; exit 1; }

[ -f "src/day$1.rs" ] && { echo "src file exists"; exit 1; }
[ -f "input/day_$1.rs" ] && { echo "input file exists"; exit 1; }

export DAY="$1"
export INPUT=$(pbpaste | awk '{printf "%s\\n", $0}')

[ -z INPUT ] && { echo "expected input in clipboard"; exit 1; }

SRC_FILE=src/day$1.rs
INP_FILE=input/day_$1.txt
MAIN_FILE=src/main.rs

# create the files
cat day.rs.tmpl | envsubst > $SRC_FILE
echo "UPDATE ME IN ${INP_FILE}" > $INP_FILE

# update the referenced day in main.rs
replace='s/day[0-9]*/day'$DAY'/g'
sed -i '' -e $replace $MAIN_FILE

# stage the changes
git add $SRC_FILE $MAIN_FILE

# test we still compile
if ! cargo test ; then
    echo 'There was an issue creating the files.'
    exit 1
fi

# print results
cat << EOF
created:
    ${SRC_FILE}
    ${INP_FILE}
updated:
    ${MAIN_FILE}
EOF