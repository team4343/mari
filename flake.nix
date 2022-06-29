{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = inputs:
    inputs.nci.lib.makeOutputs {
      root = ./.;

      overrides.shell = common: prev: {
        packages = prev.packages ++ (with common.pkgs; [(pkgs.callPackage ./nix/wpilib-toolchain.nix {}) cmake gnumake rust-analyzer]);
        env = prev.env ++ [
          { name = "PROTOBUF_LOCATION"; eval = "${common.pkgs.protobuf}"; }
          { name = "PROTOC";            eval = "$PROTOBUF_LOCATION/bin/protoc"; }
          { name = "PROTOC_INCLUDE";    eval = "$PROTOBUF_LOCATION/include"; }
        ];
      };
    };
}
