# Genesis Protocol - Deployment Version

## Estrutura do Repositório para Deploy

Este repositório foi otimizado para deploy, contendo apenas os arquivos essenciais para produção.

### Arquivos Essenciais Mantidos

#### Configuração do Projeto
- `Cargo.toml` - Configuração principal do projeto Rust
- `pyproject.toml` - Configuração para bindings Python
- `Cargo.lock` - Lock file das dependências Rust
- `.gitignore` - Arquivos ignorados pelo Git

#### Código Fonte
- `src/` - Código fonte principal do protocolo
  - `lib.rs` - Biblioteca principal
  - `tron.rs` - Implementação do Tron (organismo digital)
  - `neural.rs` - Redes neurais
  - `dna.rs` - Estrutura de DNA digital
  - `collective.rs` - Comportamento coletivo
  - `evolution.rs` - Algoritmos de evolução
  - `network.rs` - Comunicação em rede
  - `error.rs` - Tratamento de erros

#### Documentação
- `README.md` - Documentação principal do projeto
- `PROTOCOL_SPECIFICATION.md` - Especificação técnica do protocolo

### Arquivos Movidos para `deployment-archive/`

Os seguintes arquivos foram movidos para a pasta `deployment-archive/` pois não são necessários para deploy:

#### Documentação de Desenvolvimento
- `CURRENT_STATUS.md` - Status atual do desenvolvimento
- `DEMO_EXECUTION_REPORT.md` - Relatórios de execução de demos
- `LANDING_PAGE_PROMPT.md` - Prompts para landing page
- `ROADMAP.md` - Roadmap do projeto
- `QUICK_START_STATUS.md` - Status do quick start
- `QUICK_START.md` - Guia de início rápido
- `IMPLEMENTATION_PLAN.md` - Plano de implementação

#### Exemplos e Demos
- `examples/` - Exemplos de código para desenvolvimento
- `demos/` - Demonstrações do protocolo

### Pastas de Build (Ignoradas pelo Git)
- `target/` - Artefatos de build do Rust
- `dist/` - Distribuição Python
- `.venv/` - Ambiente virtual Python

## Como Fazer Deploy

### Para Rust
```bash
cargo build --release
```

### Para Python
```bash
pip install maturin
maturin build --release
```

### Para WebAssembly
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Estrutura Final para Deploy

```
genesis-protocol/
├── src/                    # Código fonte principal
├── Cargo.toml             # Configuração Rust
├── pyproject.toml         # Configuração Python
├── Cargo.lock             # Lock file
├── .gitignore             # Git ignore
├── README.md              # Documentação principal
├── PROTOCOL_SPECIFICATION.md # Especificação técnica
├── target/                # Build artifacts (ignorado)
├── dist/                  # Distribuição (ignorado)
├── .venv/                 # Ambiente virtual (ignorado)
└── deployment-archive/    # Arquivos de desenvolvimento (ignorado)
```

## Recuperando Arquivos de Desenvolvimento

Se precisar dos arquivos de desenvolvimento, eles estão disponíveis em `deployment-archive/`:

```bash
# Para mover de volta (se necessário)
mv deployment-archive/* .
rmdir deployment-archive
```

## Notas de Deploy

1. **Tamanho Otimizado**: O repositório foi reduzido removendo documentação de desenvolvimento e exemplos
2. **Foco em Produção**: Mantém apenas código fonte e documentação essencial
3. **Build Limpo**: Pastas de build são ignoradas pelo Git
4. **Flexibilidade**: Arquivos de desenvolvimento podem ser recuperados se necessário

Este repositório está pronto para deploy em ambientes de produção, CI/CD, ou distribuição como biblioteca. 