#!/bin/bash

# Ścieżka do pliku Cargo.toml
CARGO_TOML="Cargo.toml"

# Odczytujemy aktualną wersję z Cargo.toml
VERSION=$(grep -oP '(?<=^version = ").*(?=")' $CARGO_TOML)

# Rozdzielamy wersję na części (major, minor, patch)
IFS='.' read -r -a VERSION_PARTS <<< "$VERSION"

# Zwiększamy część patch (ostatnią liczbę)
((VERSION_PARTS[2]++))

# Tworzymy nową wersję
NEW_VERSION="${VERSION_PARTS[0]}.${VERSION_PARTS[1]}.${VERSION_PARTS[2]}"

# Aktualizujemy wersję w Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" $CARGO_TOML

echo "Updated version to $NEW_VERSION"
