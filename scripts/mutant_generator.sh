#!/usr/bin/env bash

# Define the path where Major should be located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
MAJOR_JAR="$SCRIPT_DIR/major/lib/major.jar"

function bold {
    printf "\033[1${2}m$1\033[0m\n"
}

# Function to download Major if not already present
function download_major_if_needed {
  if [ ! -f "$MAJOR_JAR" ]; then
    echo "Major not found. Downloading..."
    mkdir -p "$SCRIPT_DIR"
    wget -q -O major-latest.zip "https://mutation-testing.org/major-latest.zip"
    unzip -q major-latest.zip -d "$SCRIPT_DIR"
    # Assuming the major jar is named major.jar inside the downloaded zip
    rm major-latest.zip
    echo "Downloaded Major to $SCRIPT_DIR"
  else
    echo "Major is already downloaded."
  fi
}

################################################################################
# Print a usage screen with an optional message
function usage {
  printf "$(bold "usage:") ./mutate.sh\n"
  if [ ! -z "$1" ]
  then
    printf "    $( "Reason: ")$1\n\n"
  fi
  printf "    This command generates mutants for all test subjects at once\n"
  printf "\n"
  printf "$(bold "Environment Variables")\n"
  printf "%s\n" "---------------------"
  printf "   MML: path of the compiled MML file. Default: 'mml/all.mml.bin'\n"
  printf "   MAJOR_JAR: path to the Major Javac Plugin jar file. Default: '$MAJOR_JAR'\n"
  exit 1
}

################################################################################
# Check for help option
if [ "--help" == "$1" ]; then
  usage
fi

# Download Major if needed
download_major_if_needed

# Default MML path if not set
if [ -z "$MML" ]; then
  MML="all.mml.bin"
fi

# Ensure JAVA_HOME is set correctly and points to Java 8
function ensure_java_8 {
  if ! $JAVA_HOME/bin/java -version 2>&1 | grep "1\.8\..*" >/dev/null; then
    echo "$(bold "Error: JAVA_HOME is not set to a Java 8 JDK.")"
    exit 1
  fi
}

ensure_java_8

# Generate mutants for a single Java file
function generate_mutants {
  local dir=$(realpath "$1")
  local java_file="$2"
  echo "================================================================================"
  echo "Running Major to generate mutants for $dir/$java_file"
  pushd "$dir" > /dev/null

  rm -rf mutants mutants.log major.log

  # Initialize command arguments array
  local CMD_ARGS=($JAVA_HOME/bin/javac -cp "$MAJOR_JAR")

  # Conditionally add MML option
  if [ ! -z "$MML" ] && [ -f "$MML" ]; then
    CMD_ARGS+=(-Xplugin:"MajorPlugin mml:$MML export.mutants")
  else
    CMD_ARGS+=(-Xplugin:"MajorPlugin export.mutants")
  fi

  # Add the java file to the arguments
  CMD_ARGS+=("$java_file")

  # Execute the command
  "${CMD_ARGS[@]}"

  popd > /dev/null
}


# Generate mutants for each Java file in test_subjects
function generate_mutants_for_all {
  # Save the current directory
  local current_dir=$(pwd)
  echo "Major JAR Path: $MAJOR_JAR"
  # Navigate to the parent directory of SCRIPT_DIR to access test_subjects at the root
  cd "$SCRIPT_DIR/.."
  
  find test_subjects -type f -name "*.java" | while read java_file; do
    dir=$(dirname "$java_file")
    base=$(basename "$java_file")
    generate_mutants "$dir" "$base"
  done

  # Change back to the original directory
  cd "$current_dir"
}

generate_mutants_for_all
