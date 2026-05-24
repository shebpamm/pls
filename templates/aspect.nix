{ ... }:
{
  flake.aspects = { ... }: {
    {{ name }} = {
      {%- for class in classes %}
      {{ class }} = 
        { pkgs, ... }: 
        {

        };
      {%- endfor %}
    };
  };
}
