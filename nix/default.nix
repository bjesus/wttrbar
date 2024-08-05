{
  lib,
  wttrbar,
  version,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  inherit version;
  inherit (wttrbar) buildInputs meta pname;

  src = lib.cleanSourceWith {
    filter = name: type: type != "regular" || !lib.hasSuffix ".nix" name;
    src = lib.cleanSource ../.;
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };
}
