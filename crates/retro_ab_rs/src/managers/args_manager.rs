use std::env;

use crate::erro_handle::ErroHandle;

pub struct RetroArgs {
    pub core: String,
    pub rom: String,
}

impl RetroArgs {
    pub fn new() -> Result<Self, ErroHandle> {
        let args = env::args().collect();

        let core = get_value(&args, "--core=")?;
        let rom = get_value(&args, "--rom=")?;

        Ok(Self { core, rom })
    }
}

fn get_key_and_value(arg: &str) -> (String, String) {
    let (mut key, mut value) = (String::from(""), String::from(""));

    if arg.starts_with("--") {
        let key_and_value = arg.replace("--", "");

        let k_v: Vec<&str> = key_and_value.rsplit('=').collect();

        key = k_v[1].to_string();

        value = k_v[0].to_string();
    }

    (key, value)
}

pub fn get_value(args: &Vec<String>, key: &str) -> Result<String, ErroHandle> {
    for arg in args {
        if arg.contains(key) {
            let (key, value) = get_key_and_value(&arg);

            if !key.is_empty() {
                return Ok(value);
            } else {
                return Err(ErroHandle {
                    level: crate::erro_handle::RetroLogLevel::RETRO_LOG_ERROR,
                    message: "Valor não encontrado:".to_owned() + &key,
                });
            }
        }
    }

    Err(ErroHandle {
        level: crate::erro_handle::RetroLogLevel::RETRO_LOG_ERROR,
        message: "Valor não encontrado:".to_owned() + key,
    })
}

#[test]
fn teste_get_values() -> Result<(), ErroHandle> {
    let mut args: Vec<String> = Vec::new();

    args.push("--core=test.c".to_string());
    args.push("--rom=test.r".to_string());

    let core = get_value(&args, "--core=")?;

    assert_eq!(
        core, "test.c",
        "valor esperado para 'value' == 'test.c', valor encontrado -> {:?}",
        core
    );

    let rom = get_value(&args, "--rom=")?;

    assert_eq!(
        rom, "test.r",
        "valor esperado para 'value' == 'test.r', valor encontrado -> {:?}",
        rom
    );

    Ok(())
}

#[test]
fn test_get_key_and_value() {
    let command = "--core=teste.d";

    assert_eq!(
        get_key_and_value(command),
        ("core".to_string(), "teste.d".to_string())
    );
}
