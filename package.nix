{ stdenv
, parted
, pkgconfig
, dbus
, rust
, gettext
, fetchFromGitHub
, lib
, callPackage
, darwin
, llvmPackages
, libxml2
, glib
, libunistring
, writeShellScript
, glibc
, tzdata
, nixStable
, makeWrapper
, path
, xorg
, libunwind
, openssl
, cmake
, freetype
, expat
, libffi
, readline
, icu
, zlib
, python35
, python2
, autoconf213
, automake
, libtool
, libdnet
, yasm
}:

let
  gitignoreSrc = fetchFromGitHub {
    owner = "hercules-ci";
    repo = "gitignore";
    # put the latest commit sha of gitignore Nix library here:
    rev = "2ced4519f865341adcb143c5d668f955a2cb997f";
    # use what nix suggests in the mismatch message here:
    sha256 = "sha256-X8xHVRr8N6SzI8Ju87V+A75r3ZwF+CEuXcx5nfZbhTk=";
  };
  inherit (import gitignoreSrc { inherit lib; }) gitignoreSource;
in
stdenv.mkDerivation rec {
  pname = "fireframe";
  version = "0.0.1";

  src = gitignoreSource ./.;

  cargoSha256 = "sha256-Z5LxAFc4SGFpbPRF1tL7qHSzkt8xCaVXsiidbp4QSk0=";

  nativeBuildInputs = [
    # python35
    python2
    pkgconfig
    makeWrapper
    gettext
  ];

  buildInputs = [
    xorg.libX11
    libunwind
    openssl
    cmake
    freetype
    expat
    llvmPackages.clang
    llvmPackages.llvm # llvm-objdump
    autoconf213 # automake gettext libtool
    xorg.libXt libdnet
    libffi readline icu zlib
    yasm
  ];

#  NIX_CFLAGS_COMPILE = toString ([
#    "-I${glib.dev}/include/gio-unix-2.0"
#    "-I${nss.dev}/include/nss"
#  ];


  preBuild = ''
    export LIBCLANG_PATH=${llvmPackages.libclang}/lib
    #export CFLAGS="$CFLAGS -Wno-error=format-security -Wno-error"
    #export BINDGEN_EXTRA_CLANG_ARGS="-I${parted.dev}/include -I${glibc.dev}/include -I${llvmPackages.libclang.out}/lib/clang/${llvmPackages.libclang.version}/include" # documented in the code as always... https://github.com/rust-lang/rust-bindgen/pull/1537 # but seems broken due to https://github.com/rust-lang/rust-bindgen/issues/1723
  '';

  shellHook = ''
    ${preBuild}
    export PATH="${stdenv.lib.makeBinPath []}:$PATH"
  '';

  doCheck = false;

#  installPhase = ''
#    make VENDORED=1 DEBUG=0 RELEASE=release prefix=$out install
#    wrapProgram $out/bin/distinst --append PATH : ${stdenv.lib.makeBinPath tools}
#  '';

  meta = with stdenv.lib; {
    description = "";
    homepage = "https://github.com/ssd-solar/fireframe";
    # license = licenses.mpl2;
    maintainers = with maintainers; [ mkg20001 ];
    platforms = [ "x86_64-linux" ];
  };
}
