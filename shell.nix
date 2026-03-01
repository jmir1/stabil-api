{
  pkgs ? import <nixpkgs> { },
}:
let
  yaz = pkgs.stdenv.mkDerivation rec {
    pname = "yaz";
    version = "5.37.0";

    src = pkgs.fetchurl {
      url = "https://download.indexdata.com/pub/yaz/yaz-${version}.tar.gz";
      sha256 = "sha256-klf+sG4v27/Ot9BAwTn6E5V8TR67pqopOm3RPKsiJc4=";
    };

    nativeBuildInputs = with pkgs; [
      automake
      autoconf
      libtool
      pkg-config
    ];

    buildInputs = with pkgs; [
      libxml2
      libxslt
      icu
      readline
    ];

    configureFlags = [
      "--enable-shared"
      "--with-xml2"
      "--with-xslt"
      "--with-icu"
    ];

    meta = with pkgs.lib; {
      description = "Z39.50/SRU/Solr client and server";
      homepage = "https://www.indexdata.com/resources/software/yaz/";
      license = licenses.bsd3;
    };
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    openssl
    pkg-config
    rust-analyzer
    clang

    # yaz toolkit as defined above
    yaz
    libxslt
    libxml2
  ];
  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
