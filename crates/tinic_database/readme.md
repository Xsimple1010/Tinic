# Tinic Database

O **Tinic Database** é um módulo criado para tornar o uso de bancos de dados de jogos muito mais simples 
para desenvolvedores de frontends.

Os arquivos **RDB do RetroArch**, apesar de completos, não são nada amigáveis para leitura, busca 
e integração em aplicações modernas. O **Tinic Database** resolve isso aproveitando a função de leitura de RDBs 
que o **Tinic Super** fornece. Quando os dados forem enviados pelo **Tinic Super** você pode usar o **Tinic Database** 
para guardar os dados em um banco **SQLite** e aproveitar as facilidades de busca e leitura do **SQLite**.

---

## 🎯 Objetivo

Fornecer uma camada de acesso a dados de jogos que seja:

- Simples de integrar
- Rápida para consultas
- Amigável para desenvolvedores
- Independente do formato RDB original

---

## 🦀 Suporte atual

Atualmente, o uso planejado do Tinic Database está focado em **Rust**, 
com APIs pensadas para serem fáceis de usar dentro do ecossistema do Tinic.
Se estive usando outras linguagens, terá que criar o seu próprio bando de dados de sua preferência.

Se estive usando o flutter e não quiser criar um banco de dados usando LIBs do ecossistema nativo do flutter, 
você pode usar o [Rinf](https://rinf.cunarist.org/) e aproveitar o Tinic Database. Se precisar de um 
exemplo de como isso efeito veja o [Retronic](https://github.com/Xsimple1010/retronic/tree/master/native),