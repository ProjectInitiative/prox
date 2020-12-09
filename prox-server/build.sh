#!/usr/bin/env bash
sdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

versionNum="$(curl "http://git.projecti.org/projects/devops-build-tools/raw/devops.py" | python3 - --getversion "$sdir" "$sdir/FILE-WITH-VERSION-NUM")"

target="x86_64-unknown-linux-musl"
publishDir="$sdir/target/$target/release"
artifacts="$sdir/target/$target/artifacts"

mkdir -p "$artifacts/dependencies"

cp "$publishDir/prox-server" "$artifacts/dependencies"
cp -r "$sdir/linux-installation-files/"* "$artifacts/dependencies"

makeself "$artifacts/dependencies" "$artifacts/prox-server-$versionNum.run" "prox-server installer for linux" "./prox-server-install.sh"