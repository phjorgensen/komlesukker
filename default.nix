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
  cargoHash = "sha256-U2u+9Dz8jqjeDMyO27uCAGxmv6mUkbpCNIDCUmKNqxs=";
}
