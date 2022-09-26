use crate::Result;
use crate::fs;
use crate::Jar;
use pollster::*;

#[derive(Default)]
pub struct Stove {
    recipes: Vec<Recipe>,
}

impl Stove {
    pub fn new() -> Result<Self> {
        #[cfg(target_arch = "wasm32")]
        return Err("Cooking assets in a web environment is unsupported as of the moment".to_owned());
        Ok(Self::default())
    }
    pub fn with_recipe(mut self, r: Recipe) -> Self {
        self.recipes.push(r);
        self
    }

    pub fn cook(&self, path: impl Into<String>, out: impl Into<String>) {
        let path = path.into();
        let out = out.into();

        for entry in walkdir::WalkDir::new(&path) {
            // Get entry info
            let entry = entry.expect("Could not get walkdir entry");
            let entry_path = entry.path();

            // Get file extension
            if !entry_path.is_file() { continue; }
            let fmt = match entry_path.extension() {
                Some(e) => e.to_str().unwrap(),
                None => continue,
            };

            // Create out path
            let out_path = entry_path.with_extension("jam");
            let out_path = out_path.to_str().unwrap();
            let out_path = out_path.replace(path.as_str(), out.as_str());

            // Get recipe
            let recipe = match self.get_recipe_by_format(fmt) {
                Some(r) => r,
                None => continue,
            };

            // Load file data
            let data = fs::read_to_bytes(entry_path.to_str().unwrap())
                .block_on()
                .expect("Could not open file");
            
            // Cook jam
            let jar = recipe.cook(data);
            fs::save(out_path, jar.pack())
                .block_on();
        }
    }

    fn get_recipe_by_format(&self, fmt: impl Into<String>) -> Option<&Recipe> {
        let fmt = fmt.into();
        for r in &self.recipes {
            if r.has_format(&fmt) {
                return Some(r);
            }
        }
        None
    }
}

pub struct Recipe {
    pub(crate) f: fn (Vec<u8>) -> Jar,
    pub(crate) formats: Vec<String>,
}

impl Recipe {
    pub fn new(f: fn (Vec<u8>) -> Jar, formats: Vec<impl Into<String>>) -> Self {
        let formats = {
            let mut baked: Vec<String> = Vec::new();
            for fmt in formats {
                baked.push(fmt.into());
            }
            baked
        };

        Self {
            f,
            formats,
        }
    }

    pub(crate) fn cook(&self, data: Vec<u8>) -> Jar {
        let f = self.f;
        f(data)
    }

    pub fn has_format(&self, fmt: impl Into<String>) -> bool {
        let fmt = fmt.into();
        self.formats.contains(&fmt)
    }
}