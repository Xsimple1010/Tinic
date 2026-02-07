# Tinic

Um simples reprodutor de núcleos libreto

## Observação

Este projeto ainda esta em fase inicial então muita coisa ainda pode muda e ser melhorada. tinic é dividido em 3 (tres) projetos cada qual com sua responsabilidade.

### [Retro_ab_rs](https://github.com/Xsimple1010/retro_ab_rs)

Todas as ligações aos núcleos são criadas aqui.

### [Retro_ab_av](https://github.com/Xsimple1010/retro_ab_av)

Lida com renderização e a reprodução de audio.

### [Retro_ab_gamepad](https://github.com/Xsimple1010/retro_ab_gamepad)

Gerencia os controles conectados.

## Exemplo

Primeiro instale o sdl_rs (se tiver dúvidas pode seguir o passo a passo fornecido pelos desenvolvedores do projeto [aqui!](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#windows-msvc)). com o sdl instalado agora você pode executar ``cargo run --example example -- --core=caminho para o core --rom=caminho para a rom``.

## O que esperar para as próximas versões?

- Suporte a aceleração de hardware
- Suporte a comando enviados pelo teclado
