use std::sync::RwLock;

use crate::retro_sys::retro_hw_context_type;

#[derive(Debug)]
pub struct GraphicApi {
    #[doc = " Which API to use. Set by libretro core."]
    pub context_type: retro_hw_context_type,

    #[doc = " Set by frontend.\n TODO: This is rather obsolete. The frontend should not\n be providing preallocated framebuffers."]
    pub fbo: RwLock<Option<usize>>,

    #[doc = " Set if render buffers should have depth component attached.\n TODO: Obsolete."]
    pub depth: RwLock<bool>,

    #[doc = " Set if stencil buffers should be attached.\n TODO: Obsolete."]
    pub stencil: RwLock<bool>,

    #[doc = " Use conventional bottom-left origin convention. If false,
    standard libretro top-left origin semantics are used.
    TODO: Move to GL specific interface."]
    pub bottom_left_origin: RwLock<bool>,

    #[doc = " Major version number for core GL context or GLES 3.1+."]
    pub major: RwLock<u32>,

    #[doc = " Minor version number for core GL context or GLES 3.1+."]
    pub minor: RwLock<u32>,

    #[doc = " If this is true, the frontend will go very far to avoid\n resetting context in scenarios like toggling full_screen, etc. TODO: Obsolete? Maybe frontend should just always assume this ..."]
    pub cache_context: RwLock<bool>,

    #[doc = " Creates a debug context."]
    pub debug_context: RwLock<bool>,
}

impl Default for GraphicApi {
    fn default() -> Self {
        GraphicApi {
            context_type: retro_hw_context_type::RETRO_HW_CONTEXT_NONE,
            fbo: RwLock::new(None),
            depth: RwLock::new(false),
            stencil: RwLock::new(false),
            bottom_left_origin: RwLock::new(false),
            major: RwLock::new(0),
            minor: RwLock::new(0),
            cache_context: RwLock::new(false),
            debug_context: RwLock::new(false),
        }
    }
}

impl GraphicApi {
    pub fn with(context_type: retro_hw_context_type) -> Self {
        Self {
            context_type,
            ..Default::default()
        }
    }
}
