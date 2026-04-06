{ ... }:
{
  flake.aspects = { ... }: {
    {{ name }} = {
      nixos = { pkgs, ... }: {

      };
      homeManager = { pkgs, ... }: {

      };
    };
  };
}
