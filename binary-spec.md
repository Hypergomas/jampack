<div align="center">

# JamPack binary specification

</div>

&nbsp;&nbsp;&nbsp;&nbsp;
All JamPack-encoded data is stored in a binary file called a Jar (or a ```.jam``` file). This document describes the way in which all jars must be encoded.

&nbsp;&nbsp;&nbsp;&nbsp;
In the beginning of every ```.jam``` file, there is a header. This header contains essential metadata for the decoders.

```
[CATEGORY] [TYPE] (DATA)
|_______________|
     Header
```

&nbsp;&nbsp;&nbsp;&nbsp;
The ```CATEGORY``` describes what group the asset belongs in (i.e. image, audio), while the ```TYPE``` describes which kind of asset it is (i.e. RGB, RGBA). Both are stored as singular bytes in memory.

&nbsp;&nbsp;&nbsp;&nbsp;
The ```DATA``` composes all subsequent bytes after the header.

<div align="center">

## In practice

</div>

```Rust
use jampack::{Jam, Jar};

fn main() {

}

struct Ferret {
    pub name: String,
    pub age: u8,
}

impl Jam for Ferret {
    fn encode(&self) -> Jar {
        let formatted = format!("{}(=SEPARATOR=){}", self.name, self.age);
        Jar::new(
            Self::jam_type_idx(),
            0,
            formatted.as_bytes().to_vec()
        )
    }

    fn decode(ty: u8, data: Vec<u8>) -> jampack::Result<Self> {
        let data = String::from_utf8_lossy(data);
        let data: Vec<&str> = data.split("(=SEPARATOR=)").collect();

        if data.len != 2 {
            return Err("Invalid argument count for Ferret!".to_owned());
        }

        Ok(Self {
            name: data[0].to_owned(),
            age: data[1].parse::<u8>().unwrap(),
        })
    }

    fn jam_type_idx() -> u8 {
        128
    }
}
```

<div align="center">

## Built-in types

</div>

&nbsp;&nbsp;&nbsp;&nbsp;
As of v0.1.0, JamPack comes with 2 built-in asset types.

<div align="center">

**Bytes (Category 0)**

| Type | Value |
|:----:|:-----:|
| 0 | Bytes |

**Image (Category 1)**

| Type | Value |
|:----:|:-----:|
| 0 | RGB |
| 1 | RGBA |

</div>