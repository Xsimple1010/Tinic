# Tinic

Um simples reprodutor de núcleos libreto

## Observação

Este projeto está em fase inicial então muita coisa ainda pode mudar e ser melhorada.
Tinic é dividido em 3 (três) projetos, cada qual com sua responsabilidade.

### [Retro_core](./crates/retro_core)

Todas as ligações aos núcleos são criadas aqui.

### [Retro_av](./crates/retro_av)

Lida com renderização e a reprodução de áudio.

### [Retro_controllers](./crates/retro_controllers)

Gerencia os controles conectados.

## Exemplo

Agora basta executar ``cargo run --example tinic_example -- --core=./cores/test.dll --rom=./roms/test.smc``.

## O que esperar para as próximas versões?

- Criar uma documentação decente.
- Suporta comando enviados pelo teclado.
- Lidar melhor com os casos de erros em todos os projetos.
