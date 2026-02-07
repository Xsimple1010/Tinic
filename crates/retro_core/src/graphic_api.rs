use libretro_sys::binding_libretro::{retro_hw_context_type, retro_hw_render_callback};
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    RwLock,
};

#[derive(Debug)]
pub struct GraphicApi {
    #[doc = " Which API to use. Set by libretro core."]
    pub context_type: retro_hw_context_type,

    #[doc = " Set by frontend.\n TODO: This is rather obsolete. The frontend should not\n be providing preallocated framebuffers."]
    pub fbo: RwLock<Option<usize>>,

    #[doc = " Set if render buffers should have depth component attached.\n TODO: Obsolete."]
    pub depth: AtomicBool,

    #[doc = " Set if stencil buffers should be attached.\n TODO: Obsolete."]
    pub stencil: AtomicBool,

    #[doc = " Use conventional bottom-left origin convention. If false,
    standard libretro top-left origin semantics are used.
    TODO: Move to GL specific interface."]
    pub bottom_left_origin: AtomicBool,

    #[doc = " Major version number for core GL context or GLES 3.1+."]
    pub major: AtomicU32,

    #[doc = " Minor version number for core GL context or GLES 3.1+."]
    pub minor: AtomicU32,

    #[doc = " If this is true, the frontend will go very far to avoid\n resetting context in scenarios like toggling full_screen, etc. TODO: Obsolete? Maybe frontend should just always assume this ..."]
    pub cache_context: AtomicBool,

    #[doc = " Creates a debug context."]
    pub debug_context: AtomicBool,
}

impl Default for GraphicApi {
    fn default() -> Self {
        GraphicApi {
            context_type: retro_hw_context_type::RETRO_HW_CONTEXT_NONE,
            fbo: RwLock::new(None),
            depth: AtomicBool::new(false),
            stencil: AtomicBool::new(false),
            bottom_left_origin: AtomicBool::new(false),
            major: AtomicU32::new(0),
            minor: AtomicU32::new(0),
            cache_context: AtomicBool::new(false),
            debug_context: AtomicBool::new(false),
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

    /// # Safety
    ///
    /// Garanta que o ponteiro *hw_raw_ptr* é valido antes de envia para essa função.
    pub unsafe fn try_update_from_raw(&self, hw_raw_ptr: *const retro_hw_render_callback) -> bool {
        if hw_raw_ptr.is_null() {
            return false;
        }

        let hw = unsafe { *hw_raw_ptr };

        self.depth.store(hw.depth, Ordering::SeqCst);
        self.stencil.store(hw.stencil, Ordering::SeqCst);
        self.bottom_left_origin
            .store(hw.bottom_left_origin, Ordering::SeqCst);
        self.minor.store(hw.version_minor, Ordering::SeqCst);
        self.major.store(hw.version_major, Ordering::SeqCst);
        self.cache_context.store(hw.cache_context, Ordering::SeqCst);
        self.debug_context.store(hw.debug_context, Ordering::SeqCst);

        true
    }
}
