<div align="center">

![JamPack Banner](./banner.png)

</div>

&nbsp;&nbsp;&nbsp;&nbsp;
JamPack is an extensible asset framework made for games and apps. You can cook files into jars by making recipes!

&nbsp;&nbsp;&nbsp;&nbsp;
To decode assets at runtime, you can open a jar, which you can unpack in order to receive it's raw asset data.

```Rust
fn main() {
    let my_asset = jampack::jar("res/image")
        .unwrap();
    let data = my_asset.unpack();
}
```

&nbsp;&nbsp;&nbsp;&nbsp;
You can also automatically decode it, if the provided type implements the ```Jam``` trait.

```Rust
use jampack::Jam;
fn main() {
    let my_asset = jampack::open<MyType>("res/my_asset")
        .unwrap();
}

struct MyType {}
impl Jam for MyType {
    fn unjar(ty: u8, data: Vec<u8>) -> jampack::Result<Self> {
        Ok(MyType {})
    }
}
```

&nbsp;&nbsp;&nbsp;&nbsp;
Encoding data is done using a stove, which uses recipes in order to cook asset files. If the original asset hasn't been modified, it will skip it. It will also automatically remove deleted files.

&nbsp;&nbsp;&nbsp;&nbsp;
The recommended use of a stove is in build scripts.

```Rust
// build.rs
const INPUT_DIR: &str = "./res-src";
const OUT_DIR: &str   = "./res";

fn main() {
    let stove = jampack::Stove::new()
        .unwrap()
        .with_recipe(passthrough_recipe());
    
    stove.cook(INPUT_DIR, OUT_DIR);
}

fn passthrough_recipe() -> jampack::Recipe {
    fn bake(data: Vec<u8>) -> jampack::Jar {
        jampack::Jar::new(128, 0, data)
    }
    jampack::Recipe::new(bake, vec!["bin"])
}
```

<div align="center">

**Legal**

Original jam icon used in JamPack logo made by lastspark from the Noun Project (licensed under CCBY 3.0 license)

</div>