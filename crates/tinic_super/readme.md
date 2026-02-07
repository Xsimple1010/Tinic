# 🗂️ Tinic Super

O **Tinic Super** é o módulo responsável por gerenciar todos os **recursos externos e metadados** usados pelo Tinic.

Ele não executa cores Libretro diretamente — em vez disso, atua como a **camada de gerenciamento de dados**, organizando, baixando e mantendo tudo que o Tinic precisa para funcionar.

> Se o Tinic é o console, o Tinic Super é o sistema operacional que organiza os cartuchos, capas e banco de dados.

---

## 🎯 Responsabilidade Principal

O Tinic Super cuida de **arquivos essenciais do ecossistema Tinic**, incluindo:

- 🎮 Cores Libretro  
- 🖼 Thumbnails (capas, screenshots, títulos)  
- 🗄️ Bancos de dados de jogos (RDB)  
- ℹ️ Arquivos de informação de cores (`.info`)  
- 💾 (Futuro) Save states e dados persistentes  

Ele garante que esses recursos estejam:  
✔ Organizados  
✔ Atualizados  
✔ Disponíveis localmente  
✔ Prontos para uso pelo Tinic  

---

## 🧩 O Que Ele Gerencia

### 🧠 Cores Libretro
- Download de cores compatíveis
- Organização por sistema/plataforma
- Base para controle de versões e atualizações futuras

### 🖼 Thumbnails
- Capas de jogos
- Títulos estilizados
- Screenshots
- Organização por sistema e nome do jogo

### 🗄️ RDB (Retro Database)
Bancos de dados com metadados de jogos, como:
- Nome oficial
- Desenvolvedor
- Ano de lançamento
- Gênero
- Região
- CRC para identificação automática

### ℹ️ Arquivos `.info` de Cores
Arquivos que descrevem os cores, contendo:
- Nome do sistema
- Extensões suportadas
- Necessidade de BIOS
- Suporte a save states
- Outras capacidades do core

O Tinic Super usa esses dados para que o Tinic saiba **como tratar cada core corretamente**.

---

## 🌐 Sistema de Downloads

O Tinic Super pode buscar automaticamente recursos online, como:

- 📦 Cores Libretro  
- 🗄️ Arquivos RDB  
- 🖼 Pacotes de thumbnails  
- ℹ️ Arquivos `.info`  

Isso permite que o Tinic funcione como um sistema **auto-configurável**, reduzindo a necessidade de configuração manual pelo usuário.

---

## 🧱 Estrutura de Diretórios

O Tinic Super define e gerencia a estrutura padrão de pastas usada pelo Tinic:

```
tinic/
 ├── cores/
 ├── rdb/
 ├── thumbnails/
 ├── info/
 ├── system/        (BIOS e firmwares)
 └── saves/         (futuro)
```

Isso garante organização consistente em qualquer plataforma.

---

## 🔄 Integração com o Tinic

O Tinic Super fornece ao Tinic:

| Recurso | Uso no Tinic |
|--------|---------------|
| Core | Executar jogos |
| RDB | Identificar e mostrar metadados |
| Thumbnails | Interface visual da biblioteca |
| Info | Saber como configurar o core |
| (Futuro) Save states | Continuidade do jogo |

Ele atua como a **camada de dados e suporte**, enquanto o Tinic foca na execução, interface e experiência do usuário.

---

## 💡 Filosofia

O Tinic Super existe para que o Tinic:

- Não precise se preocupar com arquivos espalhados  
- Tenha tudo organizado automaticamente  
- Seja escalável para muitos sistemas e jogos  

Ele é a **infraestrutura silenciosa** que mantém o ecossistema Tinic funcionando redondo. ⚙️✨
