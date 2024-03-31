#!/usr/bin/env bash

function PUSHD {
    pushd "$1" > /dev/null
}

function POPD {
    popd > /dev/null
}

function serialize_subject {
    echo "================================================================================"
    echo "Serializing subject $1/$2"

    base="$1"
    file="$2"

    java -cp "$SERIALIZE_JAR" serializer.peg.PegSubjectSerializer "$base" "$file"
}

function install_library {
    src=$(realpath "$1")
    trg="$2"

    if [ -z "$1" ]; then
        echo "Error: No source provided to install_library"
        exit 1
    fi

    if [ -z "$2" ]; then
        trg="$LIB"
    fi

    if [ ! -e "$LIB" ]; then
        mkdir -p "$LIB"
    fi

    if [ ! -f "$trg" ]; then # Check if the target jar file does not exist
        mv "$src" "$trg"
    else
        echo "Jar file is already installed."
    fi
}

export SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
SERIALIZER_SRC="$SCRIPT_DIR/../serializer"
SERIALIZE_JAR="$SCRIPT_DIR/serialization/serialization.jar" # The expected final jar location

# Check if the jar file is not present before running Maven commands
if [ ! -f "$SERIALIZE_JAR" ]; then
    PUSHD "$SERIALIZER_SRC"
    mvn clean package -DskipTests
    pwd
    jar_file="$(find target -name "*-jar-with-dependencies.jar")"
    install_library "$jar_file" "$SERIALIZE_JAR"
    POPD
else
    echo "Serialization jar is already present, skipping Maven package."
fi

dir="$(realpath "$(dirname "$1")")"
base="$(basename "$1")"
serialize_subject "$dir" "$base"
