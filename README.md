<div align="center">

# JamPack

</div>

&nbsp;&nbsp;&nbsp;&nbsp;
JamPack is an extensible asset pipeline made for games and apps. You can compile files into jars and decode them using the built-in encoders, or even make your own!

&nbsp;&nbsp;&nbsp;&nbsp;
By default, JamPack does not ship with any encoders or decoders, but you can enable them by enabling their respective cargo features.

```TOML
[dependencies.jampack]
version = "0.1.0"
features = [
    "image-encode", # Enables encoding images to different formats using the 'image' crate
    "image-decode" # Enables decoding raw images with the 'image' crate
]
```

&nbsp;&nbsp;&nbsp;&nbsp;
JamPack also has a binary that can be used to encode files more easily. It uses TUI to create a lightweight console interface. To enable this, all you need to do is to clone the main repository and compile it as a binary with

```Bash
git clone "https://www.github.com/WrapFX/jampack.git"
cd jampack
cargo build --release
```

or download the precompiled binaries in the repo's release page.

&nbsp;&nbsp;&nbsp;&nbsp;
To decode assets at runtime, you need to open a new Jar.

```Rust
use jampack::Jar;
fn main() {
    let my_asset = Jar::raw("res/image.jam")
        .unwrap();
}
```

You can also open it as a prebuilt type,

```Rust
use jampack::Jar;
use image::DynamicImage as Image;
fn main() {
    let my_asset = Jar::open<Image>("res/image.jam")
        .unwrap();
}
```

or implement your own types.

```Rust
use jampack::{Jar, Jam};
fn main() {
    let my_asset = Jar::open<MyType>("res/myasset.jam")
        .unwrap();
}

struct MyType {}
impl Jam for MyType {
    fn encode(data: &[u8]) -> Result<&[u8], String> {
        Ok(data)
    }
    
    fn decode(data: &[u8]) -> Result<Self, String> {
        Ok(MyType {})
    }
}
```