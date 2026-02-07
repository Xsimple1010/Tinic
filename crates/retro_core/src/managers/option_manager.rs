use crate::tools::ffi_tools::get_arc_string_from_ptr;
use crate::tools::mutex_tools::get_string_mutex_from_ptr;
use crate::{
    libretro_sys::binding_libretro::{
        retro_core_option_v2_category, retro_core_option_v2_definition, retro_core_options_v2,
        retro_core_options_v2_intl,
    },
    tools::mutex_tools::get_string_rwlock_from_ptr,
};
use generics::constants::{CORE_OPTION_EXTENSION_FILE, MAX_CORE_OPTIONS};
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::Arc;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Mutex, RwLock},
};

#[derive(Default, Debug)]
pub struct CoreValue {
    pub value: Mutex<String>,
    pub label: Arc<String>,
}

#[derive(Default, Debug)]
pub struct CoreOpt {
    pub key: Arc<String>,
    pub visibility: AtomicBool,
    pub need_update: AtomicBool,
    pub selected: RwLock<String>,
    pub desc: Arc<String>,
    pub desc_categorized: Arc<String>,
    pub info: Arc<String>,
    pub info_categorized: Arc<String>,
    pub category_key: Arc<String>,
    pub values: Mutex<Vec<CoreValue>>,
    pub default_value: Arc<String>,
}

#[derive(Default, Debug)]
pub struct Categories {
    pub key: Arc<String>,
    pub info: Arc<String>,
    pub desc: Arc<String>,
}

#[derive(Default, Debug)]
pub struct OptionManager {
    pub file_path: RwLock<PathBuf>,
    pub categories: RwLock<Vec<Categories>>,
    pub updated_count: AtomicU16,
    pub opts: Mutex<Vec<CoreOpt>>,
}

impl OptionManager {
    pub fn new(opt_path: &str, library_name: String) -> OptionManager {
        let file_path = PathBuf::from(opt_path).join(library_name + CORE_OPTION_EXTENSION_FILE);

        OptionManager {
            updated_count: AtomicU16::new(0),
            categories: RwLock::new(Vec::new()),
            file_path: RwLock::new(file_path),
            opts: Mutex::new(Vec::new()),
        }
    }

    pub fn update_opt(&self, opt_key: &str, new_value_selected: &str) {
        self.change_value_selected(opt_key, new_value_selected);
        self.write_all_options_in_file();
    }

    pub fn get_opt_value(&self, opt_key: &str) -> Option<String> {
        for core_opt in &*self.opts.lock().unwrap() {
            if !core_opt.key.clone().to_string().eq(opt_key) {
                continue;
            }

            if !core_opt.need_update.load(Ordering::SeqCst) {
                break;
            }

            self.updated_count.fetch_sub(1, Ordering::Acquire);
            core_opt.need_update.store(false, Ordering::SeqCst);

            match core_opt.selected.read() {
                Ok(selected_value) => {
                    return Some(selected_value.clone());
                }
                _ => break,
            }
        }

        None
    }

    pub fn change_visibility(&self, key: &str, visibility: bool) {
        for core_opt in &mut *self.opts.lock().unwrap() {
            if !core_opt.key.to_string().eq(key) {
                continue;
            }

            core_opt.visibility.store(visibility, Ordering::SeqCst);

            if !visibility && core_opt.need_update.load(Ordering::SeqCst) {
                core_opt.need_update.store(false, Ordering::SeqCst);
                self.updated_count.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }

    fn write_all_options_in_file(&self) {
        let file_path = self.file_path.read().unwrap().clone();
        let mut file = File::create(file_path.clone()).unwrap();

        for opt in &*self.opts.lock().unwrap() {
            let key = &*opt.key;
            let selected = opt.selected.read().unwrap().clone();

            let buf = key.to_owned() + "=" + &selected + "\n";

            let _ = file.write(buf.as_bytes());
        }
    }

    fn change_value_selected(&self, opt_key: &str, new_value_selected: &str) {
        for core_opt in &*self.opts.lock().unwrap() {
            if !core_opt.key.clone().to_string().eq(&opt_key) {
                continue;
            }

            for core_value in &*core_opt.values.lock().unwrap() {
                if *core_value.value.lock().unwrap() != new_value_selected {
                    continue;
                }

                if !core_opt.need_update.load(Ordering::SeqCst) {
                    *core_opt.selected.write().unwrap() = new_value_selected.to_string();

                    self.updated_count.fetch_add(1, Ordering::SeqCst);
                    core_opt.need_update.store(true, Ordering::SeqCst);
                }

                return;
            }
        }
    }

    fn load_all_option_in_file(&self) {
        let file_path = self.file_path.read().unwrap().clone();

        let mut file = File::open(file_path).unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let lines: Vec<&str> = buf.split('\n').collect();

        for line in &lines {
            if line.is_empty() {
                return;
            }

            let values: Vec<&str> = line.split('=').collect();

            let opt_key = values.first().unwrap();
            let value_selected = values
                .get(1)
                .expect("nao foi possível recupera o valor do arquivo de opções")
                .split_ascii_whitespace()
                .next()
                .expect("nao foi possível recupera o valor do arquivo de opções");

            self.change_value_selected(opt_key, value_selected);
        }
    }

    //TODO: adiciona um meio do usuário saber se ocorrer um erro ao tentar salva ou ler o arquivo
    pub fn try_reload_pref_option(&self) {
        let file_path = self.file_path.read().unwrap().clone();

        //se o arquivo ainda nao existe apenas
        //crie um novo arquivo e salve a configuração padrão do núcleo
        if !file_path.exists() {
            self.write_all_options_in_file();
        } else {
            self.load_all_option_in_file()
        }
    }

    //===============================================
    //=================v2_intl=======================
    //===============================================

    fn get_v2_intl_category(&self, categories: *mut retro_core_option_v2_category) {
        let categories =
            unsafe { *(categories as *mut [retro_core_option_v2_category; MAX_CORE_OPTIONS]) };

        for category in categories {
            if !category.key.is_null() {
                let key = get_arc_string_from_ptr(category.key);
                let info = get_arc_string_from_ptr(category.info);
                let desc = get_arc_string_from_ptr(category.desc);

                self.categories
                    .write()
                    .unwrap()
                    .push(Categories { key, desc, info });
            } else {
                break;
            }
        }
    }

    fn get_v2_intl_definitions(&self, definitions: *mut retro_core_option_v2_definition) {
        let definitions = unsafe { *(definitions as *mut [retro_core_option_v2_definition; 90]) };

        for definition in definitions {
            if !definition.key.is_null() {
                let key = get_arc_string_from_ptr(definition.key);
                let selected = get_string_rwlock_from_ptr(definition.default_value);
                let default_value = get_arc_string_from_ptr(definition.default_value);
                let info = get_arc_string_from_ptr(definition.info);
                let desc = get_arc_string_from_ptr(definition.desc);
                let desc_categorized = get_arc_string_from_ptr(definition.desc_categorized);
                let category_key = get_arc_string_from_ptr(definition.category_key);
                let info_categorized = get_arc_string_from_ptr(definition.info_categorized);
                let values = Mutex::new(Vec::new());
                let need_update = AtomicBool::new(false);

                for retro_value in definition.values {
                    if !retro_value.label.is_null() {
                        let value = get_string_mutex_from_ptr(retro_value.value);
                        let label = get_arc_string_from_ptr(retro_value.label);

                        values.lock().unwrap().push(CoreValue { label, value });
                    }
                }

                self.opts.lock().unwrap().push(CoreOpt {
                    key,
                    selected,
                    visibility: AtomicBool::new(true),
                    default_value,
                    info,
                    desc,
                    category_key,
                    desc_categorized,
                    info_categorized,
                    values,
                    need_update,
                })
            } else {
                break;
            }
        }
    }

    pub fn convert_option_v2_intl(&self, option_intl_v2: retro_core_options_v2_intl) {
        unsafe {
            if option_intl_v2.local.is_null() {
                let us: retro_core_options_v2 = *(option_intl_v2.us);
                self.get_v2_intl_definitions(us.definitions);
                self.get_v2_intl_category(us.categories);
            } else {
                let local: retro_core_options_v2 = *(option_intl_v2.local);
                self.get_v2_intl_definitions(local.definitions);
                self.get_v2_intl_category(local.categories);
            }
        }
    }
    //===============================================
}
