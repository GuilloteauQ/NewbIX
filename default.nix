with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "sifa";
  version = "1.0";
  # src = ./.;
  src = fetchTarball("https://github.com/GuilloteauQ/SIFA/tarball/main");
  buildInputs = [ cargo ];

  checkPhase = "";
  cargoSha256 = "sha256:0fsg7w2cwq9mv2467d21p81yb60j0w0a1ap07aq5x3fj9bd9y61r";

}

