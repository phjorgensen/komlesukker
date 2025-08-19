{
  rustPlatform,
  openssl,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  name = "komlesukker";
  src = ./.;

  buildInputs = [
    openssl
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  # cargoLock.lockFile = ./Cargo.lock;
  # cargoHash = pkgs.lib.fakeHash;
  cargoHash = "sha256-VLYSWKa/Np0c4/5IfBTJZDVtAr8Nt7DMDcEOOc0sLa8=";
}
