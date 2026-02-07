use crate::core::CoreWrapperIns;
use crate::graphic_api::GraphicApi;
use crate::{core::CoreWrapper, environment::RetroEnvCallbacks};
use generics::erro_handle::ErroHandle;
use generics::retro_paths::RetroPaths;
use libretro_sys::binding_libretro::retro_log_level::RETRO_LOG_ERROR;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

static CONTEXTS: Mutex<Vec<Arc<RetroContext>>> = Mutex::new(Vec::new());

pub type RetroCtxIns = Arc<RetroContext>;

pub struct RetroContext {
    pub id: Uuid,
    pub core: CoreWrapperIns,
}

impl Drop for RetroContext {
    fn drop(&mut self) {
        let _ = self.core.de_init();
    }
}

impl RetroContext {
    pub fn new(
        core_path: &str,
        paths: RetroPaths,
        callbacks: RetroEnvCallbacks,
        graphic_api: GraphicApi,
    ) -> Result<RetroCtxIns, ErroHandle> {
        let id = Uuid::new_v4();

        let context = Arc::new(RetroContext {
            id,
            core: CoreWrapper::new(id, core_path, paths.clone(), callbacks, graphic_api)?,
        });

        context.core.init()?;

        match &mut CONTEXTS.lock() {
            Ok(ctx) => {
                ctx.push(context.clone());
            }
            Err(e) => {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: e.to_string(),
                })
            }
        }

        Ok(context)
    }

    pub fn is_valid(&self) -> bool {
        let mut is_valide = false;

        if let Ok(contexts) = &CONTEXTS.lock() {
            for ctx in contexts.iter() {
                if ctx.id.eq(&self.id) {
                    is_valide = true;
                    break;
                }
            }
        }

        is_valide
    }

    pub fn delete(&self) -> Result<(), ErroHandle> {
        match &mut CONTEXTS.lock() {
            Ok(contexts) => {
                let position = contexts.partition_point(|ctx| ctx.id == self.id);

                if !contexts.is_empty() {
                    contexts.remove(position - 1);
                }
                Ok(())
            }
            Err(e) => Err(ErroHandle {
                level: RETRO_LOG_ERROR,
                message: e.to_string(),
            }),
        }
    }

    pub fn get_num_contexts() -> usize {
        if let Ok(contexts) = &CONTEXTS.lock() {
            contexts.len()
        } else {
            0
        }
    }

    #[doc = "
        # Pegar uma instância pelo seu id

        Use isso com moderação, pois pode causar muita confusão no código.
    "]
    pub fn get_from_id(id: &Uuid) -> Result<RetroCtxIns, ErroHandle> {
        if let Ok(contexts) = &CONTEXTS.lock() {
            for ctx in contexts.iter() {
                if ctx.id.eq(id) {
                    return Ok(ctx.clone());
                }
            }
        }

        Err(ErroHandle {
            message: "O contexto voce esta tentando acessar não existe".to_string(),
            level: RETRO_LOG_ERROR,
        })
    }
}

#[cfg(test)]
mod retro_context {
    use crate::retro_context::RetroContext;
    use crate::test_tools::context::get_context;
    use generics::erro_handle::ErroHandle;

    #[test]
    fn test_create_and_delete() -> Result<(), ErroHandle> {
        let ctx = get_context()?;

        assert_eq!(
            ctx.is_valid(),
            true,
            "O contexto id -> {:?} nao foi inicializado!",
            ctx.id
        );

        let current_id = ctx.id.clone();

        ctx.delete()?;

        assert_eq!(
            ctx.is_valid(),
            false,
            "O contexto id -> {:?} nao foi removido!",
            current_id
        );

        Ok(())
    }

    #[test]
    fn get_from_id() -> Result<(), ErroHandle> {
        let ctx = get_context()?;

        let same_ctx = RetroContext::get_from_id(&ctx.id)?;

        assert_eq!(same_ctx.id, ctx.id);

        Ok(())
    }
}
