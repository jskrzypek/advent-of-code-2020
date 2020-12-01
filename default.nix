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
in mkShell {
  name = "advent-of-code-2020";
  buildInputs = [ cargo-aoc clojure ];
}