with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "sifa";
  version = "1.0";
  # src = ./.;
  src = fetchTarball("https://github.com/GuilloteauQ/SIFA/tarball/main");
  buildInputs = [ cargo ];

  checkPhase = "";
  cargoSha256 = "sha256:0ilva42mnqkbpwp646zwrahlpnkfr3q4b7xcjyi2zwgsw6fqbdcx";

}

