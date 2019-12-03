with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "RustyBot";
  buildInputs = [
    pkgs.openssl
  ];
  shellHook = ''
    export OPENSSL_DIR="${openssl.dev}"
    export OPENSSL_LIB_DIR="${openssl.out}/lib"
  '';
}
