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
  cargoHash = "sha256-SFHFRWBK+C7ZkGM8wcXQs4RUdSPKk0fHZQkYnl0Mpfk=";
}
