#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

declare -a arr=("skill_polling" "top_players_polling")

readonly TARGET_HOST=raspberrypi.local
readonly TARGET_PATH=~/skill_polling

rsync Cargo.toml ${TARGET_HOST}:~/runesync-backend/Cargo.toml
ssh -t ${TARGET_HOST} rm -rf ~/runesync-backend/src
rsync -a src/* ${TARGET_HOST}:~/runesync-backend/src
ssh -t ${TARGET_HOST} cd ~/runesync-backend; cargo build

for bin in "${arr[@]}"
do
    ssh -t ${TARGET_HOST} sudo systemctl restart ${bin}.service
done