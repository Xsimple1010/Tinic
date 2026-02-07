# Tinic

Um simples reprodutor de núcleos libreto

## Observação

Este projeto ainda está em fase inicial então muita coisa ainda pode mudar e ser melhorada.
Tinic é dividido em 3 (três) projetos, cada qual com sua responsabilidade.

### [Retro_ab_rs](./crates/retro_ab_rs)

Todas as ligações aos núcleos são criadas aqui (depende do SDL2).

### [retro_av](./crates//retro_av)

Lida com renderização e a reprodução de áudio (depende do SDL2).

### [Retro_ab_gamepad](./crates/retro_controllers)

Gerencia os controles conectados.

## Exemplo

Primeiro instale o "sdl_rs" (se tiver dúvidas pode seguir o passo a passo fornecido pelos desenvolvedores do
projeto [aqui!](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#windows-msvc)).

Agora basta executar ``cargo run --example tinic_example -- --core=./cores/test.dll --rom=./roms/test.smc``.

## O que esperar para as próximas versões?

- Criar uma documentação decente.
- Suporta comando enviados pelo teclado.
- Lidar melhor com os casos de erros em todos os projetos.
