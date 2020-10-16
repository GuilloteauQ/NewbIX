let
    pkgs = import <nixpkgs> {};
    stdenv = pkgs.stdenv;
in with pkgs; {
  newbix = stdenv.mkDerivation {
    name = "newbix";
    pname = "newbix";
    version = "1.0";
    src = ./.;
    buildInputs = [
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
