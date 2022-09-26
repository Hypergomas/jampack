<div align="center">

# JamPack

</div>

&nbsp;&nbsp;&nbsp;&nbsp;
JamPack is an extensible asset pipeline made for games and apps. You can compile files into jars using the built-in encoders, or even your own!

&nbsp;&nbsp;&nbsp;&nbsp;
By default, JamPack does not ship with any encoders, but you can enable them by selecting their respective cargo features.

```TOML
[dependencies.jampack]
version = "0.1.0"
features = [
    "image", # Enables encoding images to different formats using the 'image' crate
    ...
]
```

&nbsp;&nbsp;&nbsp;&nbsp;
To decode assets at runtime, you can open a jar, which you can unpack in order to receive it's raw asset data.

```Rust
fn main() {
    let my_asset = jampack::jar("res/image.jam")
        .unwrap();
    let data = my_asset.unpack();
}
```

&nbsp;&nbsp;&nbsp;&nbsp;
You can also automatically decode it, if the provided type implements the ```Jam``` trait.

```Rust
use jampack::Jam;
fn main() {
    let my_asset = jampack::open<MyType>("res/myasset.jam")
        .unwrap();
}

struct MyType {}
impl Jam for MyType {
    fn decode(data: Vec<u8>) -> jampack::Result<Self> {
        Ok(MyType {})
    }
}
```

&nbsp;&nbsp;&nbsp;&nbsp;
Encoding data is done using an stove, which uses recipes in order to cook asset files. If the original asset hasn't been modified, it will skip it.

&nbsp;&nbsp;&nbsp;&nbsp;
The recommended use of packagers is in build scripts.

```Rust
// build.rs
use jampack::{Stove, Recipe};
use jampack::builtin::image_recipe;

fn main() {
    let stove = Stove::new()
        .with_recipe(image_recipe());
    
    stove.cook("./res-src", "./res");
}
```