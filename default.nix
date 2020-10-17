with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "sifa";
  version = "1.0";
  # src = ./.;
  src = fetchTarball("https://github.com/GuilloteauQ/SIFA/tarball/main");
  buildInputs = [ cargo ];

  checkPhase = "";
  cargoSha256 = "sha256:0z1m63xi0fll1gg2x7ynykfis8n3zydwavf9pjxzcsh6c8pa8i77";

}

