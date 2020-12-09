#!/usr/bin/env bash
sdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

versionNum="$(curl "http://git.projecti.org/projects/devops-build-tools/raw/devops.py" | python3 - --getversion "$sdir" "$sdir/FILE-WITH-VERSION-NUM")"
description="prox-server installer for linux"


target="x86_64-unknown-linux-musl"
publishDir="$sdir/target/$target/release"
artifacts="$sdir/target/$target/artifacts"

mkdir -p "$artifacts/dependencies"

# Build the universal installer

cp "$publishDir/prox-server" "$artifacts/dependencies"
cp -r "$sdir/linux-installation-files/"* "$artifacts/dependencies"

makeself "$artifacts/dependencies" "$artifacts/prox-server-$versionNum.run" "$description" "./prox-server-install.sh"

# Build the .deb package
DEBIAN="$artifacts/prox_server_$versionNum/DEBIAN"
mkdir -p "$DEBIAN"

mkdir -p "$DEBIAN/usr/local/bin/"
cp "$artifacts/dependencies/prox-server" "$DEBIAN/usr/local/bin/"
mkdir -p "$DEBIAN/etc/systemd/system/"
cp "$artifacts/dependencies/prox-server-daemon.service" "$DEBIAN/etc/systemd/system/"

cat << 'EOF' >> "$DEBIAN/control"
Package: prox-server
Version: $(versionNum)
Architecture: all
Essential: no
Priority: optional
Maintainer: Kyle Petryszak
Description: $(description)
EOF
# Depends: packages;my-program;needs;to;run

# TODO: add in postinit to enable the service
dpkg-deb --build "$artifacts/prox_server_$versionNum"
