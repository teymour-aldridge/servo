/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::{WebGLExtension, WebGLExtensionSpec, WebGLExtensions};
use canvas_traits::webgl::WebGLVersion;
use crate::dom::bindings::codegen::Bindings::EXTColorBufferHalfFloatBinding;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::webgl_extensions::ext::oestexturehalffloat::OESTextureHalfFloat;
use crate::dom::webglrenderingcontext::WebGLRenderingContext;
use dom_struct::dom_struct;

#[dom_struct]
pub struct EXTColorBufferHalfFloat {
    reflector_: Reflector,
}

impl EXTColorBufferHalfFloat {
    fn new_inherited() -> EXTColorBufferHalfFloat {
        Self {
            reflector_: Reflector::new(),
        }
    }
}

impl WebGLExtension for EXTColorBufferHalfFloat {
    type Extension = EXTColorBufferHalfFloat;
    fn new(ctx: &WebGLRenderingContext) -> DomRoot<EXTColorBufferHalfFloat> {
        reflect_dom_object(
            Box::new(EXTColorBufferHalfFloat::new_inherited()),
            &*ctx.global(),
            EXTColorBufferHalfFloatBinding::Wrap,
        )
    }

    fn spec() -> WebGLExtensionSpec {
        WebGLExtensionSpec::Specific(WebGLVersion::WebGL1)
    }

    fn is_supported(ext: &WebGLExtensions) -> bool {
        OESTextureHalfFloat::is_supported(ext)
    }

    fn enable(_ext: &WebGLExtensions) {}

    fn name() -> &'static str {
        "EXT_color_buffer_half_float"
    }
}
