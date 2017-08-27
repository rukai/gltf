
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {image, json, Gltf};

pub use json::texture::{MagFilter, MinFilter, WrappingMode};

lazy_static! {
    static ref DEFAULT_SAMPLER: json::texture::Sampler = Default::default();
}

/// A reference to a `Texture`.
#[derive(Clone, Debug)]
pub struct Info<'a> {
    /// The parent `Texture` struct.
    texture: Texture<'a>,

    /// The corresponding JSON struct.
    json: &'a json::texture::Info,
}

///  Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    /// The parent `Gltf` struct.
    #[allow(dead_code)]
    gltf: &'a Gltf,

    /// The corresponding JSON index - `None` when the default sampler.
    index: Option<usize>,

    /// The corresponding JSON struct.
    json: &'a json::texture::Sampler,
}

/// A texture and its sampler.
#[derive(Clone, Debug)]
pub struct Texture<'a> {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::texture::Texture,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub(crate) fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::texture::Sampler,
    ) -> Self {
        Self {
            gltf: gltf,
            index: Some(index),
            json: json,
        }
    }

    /// Constructs the default `Sampler`.
    pub fn default(gltf: &'a Gltf) -> Self {
        Self {
            gltf: gltf,
            index: None,
            json: &DEFAULT_SAMPLER,
        }
    }

    /// Returns the internal JSON index if this `Sampler` was explicity defined.
    ///
    /// This function returns `None` if the `Sampler` is the default sampler.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Sampler {
        self.json
    }

    /// Magnification filter.
    pub fn mag_filter(&self) -> Option<MagFilter> {
        self.json.mag_filter.map(|filter| filter.unwrap())
    }

    /// Minification filter.
    pub fn min_filter(&self) -> Option<MinFilter> {
        self.json.min_filter.map(|filter| filter.unwrap())
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// `s` wrapping mode.
    pub fn wrap_s(&self) -> WrappingMode {
        self.json.wrap_s.unwrap()
    }

    /// `t` wrapping mode.
    pub fn wrap_t(&self) -> WrappingMode {
        self.json.wrap_t.unwrap()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> Texture<'a> {
    /// Constructs a `Texture`.
    pub(crate) fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::texture::Texture,
    ) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Texture {
        self.json
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the sampler used by this texture.
    pub fn sampler(&self) -> Sampler<'a> {
        self.json.sampler
            .as_ref()
            .map(|index| self.gltf.samplers().nth(index.value() as usize).unwrap())
            .unwrap_or_else(|| Sampler::default(self.gltf))
    }

    /// Returns the image used by this texture.
    pub fn source(&self) -> image::Image<'a> {
        self.gltf.images().nth(self.json.source.value() as usize).unwrap()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}


impl<'a> Info<'a> {
    /// Constructs a reference to a `Texture`.
    pub(crate) fn new(texture: Texture<'a>, json: &'a json::texture::Info) -> Self {
        Self {
            texture: texture,
            json: json,
        }
    }

    /// Returns the internal JSON item.
    pub fn as_json(&self) ->  &json::texture::Info {
        self.json
    }

    /// The set index of the texture's `TEXCOORD` attribute.
    pub fn tex_coord(&self) -> u32 {
        self.json.tex_coord
    }

    /// Returns the referenced `Texture`.
    pub fn texture(&self) -> Texture<'a> {
        self.texture.clone()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }
}

impl<'a> AsRef<Texture<'a>> for Info<'a> {
    fn as_ref(&self) -> &Texture<'a> {
        &self.texture
    }
}
