use super::binding_libretro::retro_log_level;

#[doc = " essa callback deve ser criada no rust e envida para configure_log_interface"]
pub type RsCbT = ::std::option::Option<
    unsafe extern "C" fn(level: retro_log_level, log: *const ::std::os::raw::c_char),
>;
extern "C" {
    #[doc = " deve ser chamada para enviar fn core_log (RsCbT) para o CORE selecionado"]
    pub fn configure_log_interface(rs_cb: RsCbT, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn set_variable_value_as_null(data: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " deve ser usando em RETRO_ENVIRONMENT_GET_VARIABLE para atualizar as variáveis"]
    pub fn set_new_value_variable(
        data: *mut ::std::os::raw::c_void,
        new_value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = " enviar um diretório para o núcleo. pode ser usando nas callbacks RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY e RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY"]
    pub fn set_directory(
        data: *mut ::std::os::raw::c_void,
        new_directory: *const ::std::os::raw::c_char,
    );
}
