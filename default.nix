{ pkgs ? import <nixpkgs> { }, ... }:
with pkgs;
let
  cargo-aoc = rustPlatform.buildRustPackage rec {
    pname = "cargo-aoc";
    version = "0.3.2";

    src = fetchFromGitHub {
        owner = "jskrzypek";
        repo = pname;
        rev = "c67bf02ca216d5a64a925d8a53ad33c9822ef823";
        sha256 = "1g8281845hpdwnd5rpv4si0fhly6yyny6j87raip5qyb7lzh674x";
    };

    cargoSha256 = "1app6124ckrzn6qhcy8wm88kcwmsqvnzq8idciq3pwqp4nc8pcjw";

    nativeBuildInputs = [ pkg-config ];
    buildInputs = [ openssl ];

    doCheck = false;

    cargoBuildFlags = [ "--verbose" ];

    preBuild = ''
      substituteInPlace Cargo.toml \
        --replace ',
    "examples/boilerplate"' ""
    '';

  };

  clj2nix = callPackage (fetchFromGitHub {
    owner = "hlolli";
    repo = "clj2nix";
    rev = "89d1cda232175b588c7774e638c9ebfaaedea0e3";
    sha256 = "1xjwi3y2dylcmz9y8nx2ldghiy6qbn3f4m4l68mz1prirb2lxqi0";
    fetchSubmodules = true;
  }) { };

  common = writeShellScript "common.sh" ''
    info() {
        printf "\u001b[34;1m==>\u001b[32;1m %s\u001b[0m\n" "$1"
    }

    subinfo() {
        printf "    \u001b[34;1m>\u001b[0m %s\n" "$1"
    }

    error() {
        printf "\u001b[31;1m==>\u001b[32;1m %s\u001b[0m\n" "$1"
    }

    die() {
        error "$1"
        exit 1
    }
  '';

  get-input = writeShellScriptBin "get-input" ''
    usage_text="Usage: $0 [-y <year>] [<day> [<day> ...]]

    Options:
      -y <year>   : The year of input to fetch, 2 or 4 digits, since 2015.
                    Defaults to current year.
      <day>       : Day of input to fetch, allows multiple.
                    Defaults to fetching all available inputs."

    source ${common}

    if [[ -z $(${cargo-aoc}/bin/cargo-aoc credentials) ]]; then
      error "Credentials not set! See help:"
      ${cargo-aoc}/bin/cargo-aoc credentials -h
      die "Please set credentials and try again!"
    fi

    year="''${AOC_YEAR:-20$(date +'%y')}"

    to_epoch() {
      date ${if stdenv.isDarwin then ''-j -f "%F"'' else "-d"} "$1" +%s
    }

    date_compare() {
      local a=$(to_epoch "$1")
      local b=$(to_epoch "$2")
      return $(( a <= b ? 0 : 1 ))
    }

    while getopts ":hy:" opt; do
      case $opt in
        h)
          echo "$usage_text"
          exit 1
          ;;
        y)
          if [[ 20''${OPTARG: -2} != $year ]]; then
            if (( 2015 < 20''${OPTARG: -2} <= $(date +'%Y') )); then
              export year="20''${OPTARG: -2}"
              info "Fetching inputs for year: 20''${OPTARG: -2}"
            else
              die "Must specify a valid year when AOC exists! (2015-"$(date +'%/y')"). Received $OPTARG"
            fi
          fi
          ;;
      esac
      shift $((OPTIND - 1))
    done

    if date_compare $year-12-25 $(date +'%Y-%m-%d'); then
      max_day=25
    else
      max_day=$(date +'%d')
    fi

    declare -a days=()

    if [[ -n "$@" ]]; then
      day_args="$@"
    else
      day_args=$(seq 01 $(date +'%d'))
    fi
    for arg in $day_args; do
      if [[ 00 -le $arg ]] && [[ $arg -le $max_day ]]; then
        days="$days $(printf "%02d" "$arg")"
      else
        error "Recieved invalid day: $arg"
      fi
    done
    info "$(printf "Fetching inputs for days:"; printf " %02d" "$days")"

    cd "${builtins.toString ./.}"
    ln -sf "${builtins.toString ./.}/input" "${builtins.toString ./.}/rust/"
    for day in $days; do
      ${cargo-aoc}/bin/cargo-aoc input -d $day -y $year
      ln -sf "${builtins.toString ./.}/input/$year/day$((day)).txt" "${builtins.toString ./.}/clojure/day-$day.txt"
    done
  '';

in mkShell {
  name = "advent-of-code-2020";
  buildInputs = [ cargo-aoc clojure clj2nix get-input common ];
  passthru = { inherit cargo-aoc clj2nix get-input common; };
}