{ ... }:
{
  flake.wrappers.{{ name }} =
    { wlib, pkgs, ... }:
    {
      imports = [ wlib.modules.default ];

      config = { };
    };
}
