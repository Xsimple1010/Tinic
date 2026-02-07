# Tinic

Um simples reprodutor de núcleos libreto

## Observação

Este projeto ainda está em fase inicial então muita coisa ainda pode mudar e ser melhorada. Tinic é dividido em 3 (três) projetos, cada qual com sua responsabilidade.

### [Retro_ab_rs](https://github.com/Xsimple1010/retro_ab_rs)

Todas as ligações aos núcleos são criadas aqui (depende do SDL2).

### [Retro_ab_av](https://github.com/Xsimple1010/retro_ab_av)

Lida com renderização e a reprodução de áudio (depende do SDL2).

### [Retro_ab_gamepad](https://github.com/Xsimple1010/retro_ab_gamepad)

Gerencia os controles conectados.

## Exemplo

Primeiro instale o sdl_rs (se tiver dúvidas pode seguir o passo a passo fornecido pelos desenvolvedores do projeto [aqui!](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#windows-msvc)). Em seguida crie essa estrutura de pastas no diretório raiz do projeto.

```
// isso tem que está no diretório raiz do projeto!
retro_out_test
    |opt
    |save
    |system
```

Agora basta executar ``cargo run --example example -- --core=caminho para o core --rom=caminho para a rom``.

## O que esperar para as próximas versões?

- Criar uma documentação decente
- Suporte a comando enviados pelo teclado
- Lidar melhor com os casos de erros em todos os projetos
