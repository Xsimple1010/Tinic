use crate::constants::CORE_OPTION_EXTENSION_FILE;
use crate::{
    constants,
    retro_sys::{
        retro_core_option_v2_category, retro_core_option_v2_definition, retro_core_options_v2,
        retro_core_options_v2_intl,
    },
    tools::mutex_tools::get_string_rwlock_from_ptr,
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::RwLock,
};

#[derive(Default, Debug)]
pub struct Values {
    pub value: RwLock<String>,
    pub label: RwLock<String>,
}

#[derive(Default, Debug)]
pub struct Options {
    pub key: RwLock<String>,
    pub visibility: RwLock<bool>,
    pub selected: RwLock<String>,
    pub desc: RwLock<String>,
    pub desc_categorized: RwLock<String>,
    pub info: RwLock<String>,
    pub info_categorized: RwLock<String>,
    pub category_key: RwLock<String>,
    pub values: RwLock<Vec<Values>>,
    pub default_value: RwLock<String>,
}

#[derive(Default, Debug)]
pub struct Categories {
    pub key: RwLock<String>,
    pub info: RwLock<String>,
    pub desc: RwLock<String>,
}

#[derive(Default, Debug)]
pub struct OptionManager {
    pub file_path: RwLock<PathBuf>,
    pub categories: RwLock<Vec<Categories>>,
    pub updated: RwLock<bool>,
    pub opts: RwLock<Vec<Options>>,
}

impl OptionManager {
    pub fn new(opt_path: &str, library_name: String) -> OptionManager {
        let file_path = PathBuf::from(opt_path).join(library_name + CORE_OPTION_EXTENSION_FILE);

        OptionManager {
            updated: RwLock::new(true),
            categories: RwLock::new(Vec::new()),
            file_path: RwLock::new(file_path),
            opts: RwLock::new(Vec::new()),
        }
    }

    pub fn update_opt(&self, opt_key: &str, new_value_selected: &str) {
        self.change_value_selected(opt_key, new_value_selected);
        self.write_all_options_in_file();
    }

    pub fn change_visibility(&self, key: &str, visibility: bool) {
        for opt in &mut *self.opts.write().unwrap() {
            if opt.key.read().unwrap().eq(key) {
                *opt.visibility.write().unwrap() = visibility;
            }
        }
    }

    fn write_all_options_in_file(&self) {
        let file_path = self.file_path.read().unwrap().clone();
        let mut file = File::create(file_path.clone()).unwrap();

        for opt in &*self.opts.read().unwrap() {
            let key = opt.key.read().unwrap().clone();
            let selected = opt.selected.read().unwrap().clone();

            let buf = key + "=" + &selected + "\n";

            let _ = file.write(buf.as_bytes());
        }
    }

    fn change_value_selected(&self, opt_key: &str, new_value_selected: &str) {
        for opt in &*self.opts.read().unwrap() {
            if opt.key.read().unwrap().eq(opt_key) {
                for v in &*opt.values.read().unwrap() {
                    if *v.value.read().unwrap() == new_value_selected {
                        *opt.selected.write().unwrap() = new_value_selected.to_string();
                        *self.updated.write().unwrap() = true;
                        break;
                    }
                }

                break;
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
        let categories = unsafe {
            *(categories as *mut [retro_core_option_v2_category; constants::MAX_CORE_OPTIONS])
        };

        for category in categories {
            if !category.key.is_null() {
                let key = get_string_rwlock_from_ptr(category.key);
                let info = get_string_rwlock_from_ptr(category.info);
                let desc = get_string_rwlock_from_ptr(category.desc);

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
                let key = get_string_rwlock_from_ptr(definition.key);
                let selected = get_string_rwlock_from_ptr(definition.default_value);
                let default_value = get_string_rwlock_from_ptr(definition.default_value);
                let info = get_string_rwlock_from_ptr(definition.info);
                let desc = get_string_rwlock_from_ptr(definition.desc);
                let desc_categorized = get_string_rwlock_from_ptr(definition.desc_categorized);
                let category_key = get_string_rwlock_from_ptr(definition.category_key);
                let info_categorized = get_string_rwlock_from_ptr(definition.info_categorized);
                let values = RwLock::new(Vec::new());

                for retro_value in definition.values {
                    if !retro_value.label.is_null() {
                        let value = get_string_rwlock_from_ptr(retro_value.value);
                        let label = get_string_rwlock_from_ptr(retro_value.label);

                        values.write().unwrap().push(Values { label, value });
                    }
                }

                self.opts.write().unwrap().push(Options {
                    key,
                    visibility: RwLock::new(true),
                    selected,
                    default_value,
                    info,
                    desc,
                    category_key,
                    desc_categorized,
                    info_categorized,
                    values,
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
