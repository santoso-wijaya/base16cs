# `base16cs`

A library for defining a **palette** of **base colors** in canonical CIE
L\*a\*b\* colorspace values, and then deriving other colorspace values from
them.

This library also provides serializers and deserializers for palettes, and a
template renderer (Liquid, by default) for injecting palette and color variables
into a template for renders.

## Examples

### Define a palette, derive values, and serialize

```rust
use base16cs::{Palette, BaseColor, DerivedPalette};
use base16cs::Serializable;

// Define a canonical palette
let palette = Palette::new(
    "My Palette",
    [
        BaseColor::new("bg", 96, 0, 13),
        BaseColor::new("fg", 31, -6, -6),
    ]);

// Derive with computed sRGB values
let derived_palette = DerivedPalette::from(&palette);

// Serialize (to YAML)
let serialized = derived_palette.serialize().unwrap();
assert_eq!(serialized, r#"name: My Palette
colors:
- base:
    name: bg
    lab:
      l: 96.0
      a: 0.0
      b: 13.0
  srgb:
    red: 254
    green: 243
    blue: 218
  srgb_hex: fef3da
- base:
    name: fg
    lab:
      l: 31.0
      a: -6.0
      b: -6.0
  srgb:
    red: 56
    green: 76
    blue: 82
  srgb_hex: 384c52
"#);
```

### Load a serialized base palette, derive, then inject into a Liquid template

```rust
use base16cs::{Base16Palette, Base16DerivedPalette};
use base16cs::LiquidTemplate;

let palette_yaml = std::fs::read_to_string("/path/to/palette.yaml").unwrap();
let palette = Base16Palette::from_yaml(&palette_yaml).unwrap();

let template = LiquidTemplate::parse_file("/path/to/template.liquid", vec![]).unwrap();
// `template.render()` will take care of deriving `palette` for us.
println!(template.render(&palette, false));
```

## Liquid template render with palette injection

When a Liquid template is rendered, it will be injected with a Liquid object
keyed to `"palette"`. The Liquid object value is a serialization of a
`DerivedPalette`.

Let's say `/path/to/template.liquid` contains:

```liquid
Palette name: {{ palette.name }}
Palette colors:
{%- for color in palette.colors -%}
  {{ color.base.name }}: #{{ color.srgb_hex }}
{%- endfor -%}
```

Rendering this with the palette above would output:

```
Palette name: My Palette
Palette colors:
  bg: #fef3da
  fg: #384c52
```
