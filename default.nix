let
    moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
    stdenv = nixpkgs.stdenv;
    ruststable = (nixpkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];});
in with nixpkgs; {
  newbix = stdenv.mkDerivation {
    name = "newbix";
    pname = "newbix";
    version = "1.0";
    src = ./.;
    buildInputs = [
        ruststable
		cargo
    ];
    installPhase =''
      mkdir $out
      mkdir $out/bin
      cargo build --release
      cp ./target/release/newbix $out/bin
    '';
  };
}
