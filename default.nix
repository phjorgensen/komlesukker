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
  cargoHash = "sha256-+9mUnm+7eC2/7sTf4dCV5geWpilovd0HelbWr4PSGoQ=";
}
