use crate::Jar;

#[derive(Default)]
pub struct Stove {
    recipes: Vec<Recipe>,
}

impl Stove {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_recipe(mut self, r: Recipe) -> Self {
        self.recipes.push(r);
        self
    }

    pub fn cook(&mut self, path: impl Into<String>, out: impl Into<String>) {
        let path = path.into();
        todo!()
    }
}

pub struct Recipe {
    pub(crate) ty: u8,
    pub(crate) f: fn (Vec<u8>) -> Jar,
}

impl Recipe {
    pub fn new(ty: u8, f: fn (Vec<u8>) -> Jar) -> Self {
        Self {
            ty,
            f,
        }
    }

    pub(crate) fn cook(&self, data: Vec<u8>) -> Jar {
        let f = self.f;
        f(data)
    }
}