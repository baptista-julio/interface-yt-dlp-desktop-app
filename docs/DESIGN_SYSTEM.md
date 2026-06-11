# Design System — Padrao Visual da Aplicacao

> **Finalidade:** Este documento descreve TODOS os padroes visuais da aplicacao.
> Ao adicionar uma nova tela, componente ou funcionalidade, siga rigorosamente
> estas especificacoes para manter a consistencia visual.

## Sumario

1. [Design Tokens (CSS Custom Properties)](#1-design-tokens)
2. [Tipografia](#2-tipografia)
3. [Cores e temas](#3-cores-e-temas)
4. [Espacamento e layout](#4-espacamento-e-layout)
5. [Bordas e sombras](#5-bordas-e-sombras)
6. [Botoes](#6-botoes)
7. [Formularios (Select/Dropdown)](#7-formularios)
8. [Cards e superficies](#8-cards-e-superficies)
9. [Tabelas](#9-tabelas)
10. [Graficos SVG (Donuts)](#10-graficos-svg-donuts)
11. [Barras de progresso](#11-barras-de-progresso)
12. [Animacoes e transicoes](#12-animacoes-e-transicoes)
13. [Icones (SVG inline)](#13-icones-svg-inline)
14. [Telas simples (centralizadas)](#14-telas-simples)
15. [Tela de resultados (complexa)](#15-tela-de-resultados)
16. [Toast de notificacao](#16-toast-de-notificacao)
17. [Responsividade](#17-responsividade)
18. [Scrollbar](#18-scrollbar)
19. [Checklist para novos componentes](#19-checklist-para-novos-componentes)

---

## 1. Design Tokens

Todas as cores, sombras, raios e fontes sao definidas via CSS custom properties em `src/styles/globals.css`. **Nunca use valores hardcoded** — sempre use `var(--token)`.

### Tokens do tema claro (`:root`)

```css
/* Cores */
--primary: #094386;           /* Azul escuro ABREME — cor principal */
--primary-light: #e0ecf7;     /* Azul claro — backgrounds de destaque */
--primary-dark: #062d5a;      /* Azul mais escuro — hover do primary */
--success: #16a34a;           /* Verde — sucesso, check marks */
--success-light: #dcfce7;     /* Verde claro — backgrounds de sucesso */
--error: #dc2626;             /* Vermelho — erros */
--error-light: #fef2f2;       /* Vermelho claro — backgrounds de erro */
--warning: #f59e0b;           /* Amarelo — avisos */

/* Superficies */
--bg: #f8fafc;                /* Fundo da pagina */
--surface: #ffffff;           /* Fundo de cards, paineis */
--text: #0f172a;              /* Texto principal */
--text-secondary: #64748b;    /* Texto secundario, labels */
--border: #e2e8f0;            /* Bordas leves */
--border-dark: #cbd5e1;       /* Bordas mais fortes (hover) */

/* Sombras */
--shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.07), 0 2px 4px -2px rgba(0, 0, 0, 0.05);
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.08), 0 4px 6px -4px rgba(0, 0, 0, 0.04);

/* Raios de borda */
--radius-sm: 0.375rem;        /* 6px — botoes pequenos, badges */
--radius-md: 0.5rem;          /* 8px — botoes, inputs */
--radius-lg: 0.75rem;         /* 12px — cards, paineis */
--radius-xl: 1rem;            /* 16px — dropzone, modais */

/* Fonte */
--font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
  "Helvetica Neue", Arial, "Noto Sans", sans-serif;
```

### Tokens do tema escuro (`[data-theme="dark"]`)

```css
--primary: #2d7fd4;           /* Azul mais claro (legivel em fundo escuro) */
--primary-light: #0d2845;     /* Azul escuro — backgrounds de destaque */
--primary-dark: #094386;
--success: #22c55e;
--success-light: #052e16;
--error: #ef4444;
--error-light: #450a0a;
--bg: #0b1120;                /* Fundo escuro */
--surface: #111827;           /* Cards escuros */
--text: #e2e8f0;              /* Texto claro */
--text-secondary: #94a3b8;
--border: #1e293b;
--border-dark: #334155;
/* Sombras com maior opacidade */
--shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.4), 0 2px 4px -2px rgba(0, 0, 0, 0.3);
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.5), 0 4px 6px -4px rgba(0, 0, 0, 0.4);
```

---

## 2. Tipografia

### Font stack

```css
font-family: var(--font-family);
/* -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, ... */
```

**Nunca importe fontes externas** — o app funciona offline.

### Escala tipografica

| Elemento | Font Size | Font Weight | Line Height | Uso |
|----------|-----------|-------------|-------------|-----|
| h1 | `1.75rem` | 700 | 1.3 | Titulo principal de tela |
| h2 | `1.35rem` | 700 | 1.3 | Subtitulo, nome do ramo |
| h3 | `1.1rem` | 600 | 1.3 | Secao dentro da tela |
| Body text | `1rem` (16px base) | 400 | 1.6 | Texto corrido |
| Buttons | `0.9rem` | 500-600 | 1 | Texto de botoes |
| Labels/Captions | `0.8-0.85rem` | 500-600 | 1.3 | Labels, metadata |
| Small text | `0.75-0.78rem` | 400-500 | 1.3 | Notas, contadores |

### Regras tipograficas

- **Titulos de secao (panel titles):** `text-transform: uppercase`, `letter-spacing: 0.03-0.05em`, cor `var(--primary)`, peso 700
- **Labels de metadados:** `text-transform: uppercase`, `letter-spacing: 0.05em`, cor `var(--text-secondary)`, peso 600
- **Numeros tabulares:** sempre usar `font-variant-numeric: tabular-nums` para alinhar colunas de numeros
- **Nomes de fornecedores:** exibir em `toUpperCase()` no seletor e no header
- **Opcoes de metricas:** exibir em Title Case (primeira letra de cada palavra maiuscula)

### Responsividade tipografica

Em telas grandes (2560px+), o `html font-size` sobe para `18px`, e em 3840px+ para `20px`. Isso escala tudo proporcionalmente via `rem`.

Para font sizes em componentes inline, usar `clamp()`:

```css
/* Exemplo padrao */
font-size: clamp(MINIMO, PREFERIDO, MAXIMO);

/* Usado no app */
font-size: clamp(0.95rem, 1.1vw, 1.15rem);   /* Titulos de metrica */
font-size: clamp(0.85rem, 1vw, 1rem);          /* Texto de tabela */
font-size: clamp(0.7rem, 0.85vw, 0.85rem);     /* Tabs */
```

---

## 3. Cores e temas

### Regra fundamental

**Nunca** use cores literais (ex: `#094386`) no JSX ou CSS inline. **Sempre** use tokens:

```tsx
// CORRETO
color: "var(--primary)"
backgroundColor: "var(--surface)"

// INCORRETO
color: "#094386"
backgroundColor: "#ffffff"
```

A unica excecao e o botao "Exportar PDF" que usa `backgroundColor: "#8e44ad"` (roxo, para diferenciar do primary). Se mais cores especiais forem necessarias, crie novos tokens.

### Como funciona a troca de tema

1. `App.tsx` armazena o tema em `useState` e `localStorage`
2. `document.documentElement.setAttribute("data-theme", theme)` altera o atributo
3. As CSS variables automaticamente se adaptam via `[data-theme="dark"] { ... }`
4. O componente `ThemeToggle` fica fixo no canto superior direito

### Paleta semantica

| Token | Significado | Onde usar |
|-------|------------|-----------|
| `--primary` | Acao principal, destaque | Botoes primarios, barras, graficos, links |
| `--primary-light` | Background de destaque sutil | Background do passo ativo, arquivo selecionado |
| `--primary-dark` | Hover do primary | `:hover` de btn-primary |
| `--success` | Sucesso, completo | Check marks, toast de sucesso |
| `--error` | Erro, falha | Icone X, toast de erro, mensagens |
| `--warning` | Alerta | Avisos nao-criticos |
| `--bg` | Background da pagina | Fundo geral, linhas pares de tabela |
| `--surface` | Background de elementos | Cards, paineis, dropdowns |
| `--text` | Texto principal | Titulos, dados importantes |
| `--text-secondary` | Texto auxiliar | Labels, legendas, metadados |
| `--border` | Borda leve | Divisores, bordas de cards |
| `--border-dark` | Borda forte | Hover de inputs, scrollbar |

---

## 4. Espacamento e layout

### Container principal

```css
.app-container {
  width: 100%;
  max-width: 1400px;
  margin: 0 auto;
  padding: 1.5rem clamp(1.5rem, 3vw, 4rem);
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
```

- O container ocupa `100vh` e **nao tem scroll** (overflow hidden)
- Cada tela deve caber na viewport (exceto ResultScreen que tem scroll interno)
- O `ScreenTransition` wrapper usa `flex: 1` e `minHeight: 0` para permitir que a tela filha ocupe o espaco restante

### Padroes de gap

| Contexto | Gap | Exemplo |
|----------|-----|---------|
| Entre secoes de tela | `1.5-2rem` | Gap entre titulo e formulario |
| Dentro de card | `0.75-1rem` | Gap entre rows num card |
| Botoes lado a lado | `0.75rem` | Grupo de botoes de acao |
| Tabs | `0.35rem` | Espaco entre botoes de tab |
| Itens de estatistica | `0.15-0.35rem` | Linhas de StatRow |

### Larguras maximas de conteudo centralizado

| Componente | Max Width | Uso |
|-----------|-----------|-----|
| Telas simples (formularios) | `360-420px` | Upload, Select Sheet/Supplier |
| Dropzone | `480px` | Zona de upload de arquivo |
| Error box | `480px` | Mensagem de erro |
| Progress steps | `420px` | Lista de etapas |

---

## 5. Bordas e sombras

### Raios de borda

| Token | Valor | Onde usar |
|-------|-------|-----------|
| `--radius-sm` | `0.375rem` (6px) | Botoes de scroll, badges |
| `--radius-md` | `0.5rem` (8px) | Botoes, inputs, select |
| `--radius-lg` | `0.75rem` (12px) | Cards, paineis, header |
| `--radius-xl` | `1rem` (16px) | Dropzone |
| `9999px` | Pill | Tabs (botoes pill-shaped) |

### Sombras

| Token | Quando usar |
|-------|------------|
| `--shadow-sm` | Estado padrao de cards, botoes |
| `--shadow-md` | Hover de cards, hover de btn-primary |
| `--shadow-lg` | Elementos flutuantes (toast) |

### Bordas

- Cards e paineis: `1px solid var(--border)`
- Headers de tabela: `2px solid var(--border)` (bottom)
- Linhas de tabela: `1px solid var(--border)` (bottom)
- Divisores internos: `1px solid var(--border)` (borderTop)
- Dropzone: `2px dashed var(--border)`
- Focus ring: `0 0 0 3px rgba(9, 67, 134, 0.15)` (blue glow)

---

## 6. Botoes

### Tres variantes de botao

#### `.btn-primary` — Acao principal

```css
background-color: var(--primary);
color: white;
font-weight: 600;
box-shadow: var(--shadow-sm);
border-radius: var(--radius-md);
padding: 0.7rem 1.5rem;
font-size: 0.9rem;
```

Hover: `background: var(--primary-dark)`, `shadow-md`, `translateY(-1px)`
Active: `translateY(0)`, `shadow-sm`

#### `.btn-secondary` — Acao alternativa

```css
background-color: var(--bg);
color: var(--text);
border: 1px solid var(--border);
font-weight: 500;
```

Hover: `background: var(--border)`, `border: var(--border-dark)`

#### `.btn-outline` — Acao terciaria/voltar

```css
background-color: transparent;
color: var(--text-secondary);
border: 1px solid var(--border);
font-weight: 500;
```

Hover: `background: var(--bg)`, `color: var(--text)`

### Regras gerais de botao

- **Disabled:** `opacity: 0.5`, `cursor: not-allowed`
- **Focus visible:** `outline: 2px solid var(--primary)`, `outline-offset: 2px`
- **Transicao:** `transition: all 0.2s ease`
- **Layout:** `display: inline-flex`, `align-items: center`, `gap: 0.5rem`
- **White space:** `white-space: nowrap`

### Hierarquia visual de botoes

Na tela de resultados, a hierarquia e:

1. **Salvar Relatorio** — `btn-primary` (azul)
2. **Exportar PDF** — `btn-primary` com override `backgroundColor: "#8e44ad"` (roxo)
3. **Relatorio em Lote** — `btn-primary` com override `backgroundColor: "var(--text-secondary)"` (cinza)
4. **Abrir Pasta** — `btn-secondary`
5. **Outro Fornecedor** — `btn-outline`
6. **Recomecar** — `btn-outline`

---

## 7. Formularios

### Select/Dropdown (`.select-input`)

```css
width: 100%;
padding: 0.7rem 1rem;
font-size: 0.9rem;
color: var(--text);
background-color: var(--surface);
border: 1px solid var(--border);
border-radius: var(--radius-md);
appearance: none;
/* Seta SVG customizada como background-image */
padding-right: 2.5rem;
```

Hover: `border-color: var(--border-dark)`
Focus: `border-color: var(--primary)`, `box-shadow: 0 0 0 3px rgba(9, 67, 134, 0.15)`
Disabled: `opacity: 0.6`, `background: var(--bg)`

No tema escuro, a seta SVG e reestilizada com `stroke='%2394a3b8'` e as `option` recebem `background-color: var(--surface)`.

---

## 8. Cards e superficies

### Card padrao

```css
background-color: var(--surface);
border: 1px solid var(--border);
border-radius: var(--radius-lg);
padding: 1.5rem;
box-shadow: var(--shadow-sm);
```

Hover: `box-shadow: var(--shadow-md)`

### Painel (usado no ResultScreen e MetricsTable)

```tsx
const panelStyle: React.CSSProperties = {
  backgroundColor: "var(--surface)",
  borderRadius: "var(--radius-lg)",
  padding: "clamp(0.6rem, 1.2vh, 1rem) clamp(0.75rem, 1.5vw, 1.25rem)",
  boxShadow: "var(--shadow-sm)",
  border: "1px solid var(--border)",
};
```

### Titulo de painel

```tsx
const sectionTitleStyle: React.CSSProperties = {
  fontSize: "clamp(0.78rem, 0.88vw, 0.95rem)",
  fontWeight: 700,
  color: "var(--primary)",
  textTransform: "uppercase",
  letterSpacing: "0.04em",
  marginBottom: "clamp(0.35rem, 0.6vh, 0.5rem)",
};
```

---

## 9. Tabelas

### Headers (`<th>`)

```css
padding: clamp(0.3rem, 0.55vh, 0.45rem) clamp(0.4rem, 0.8vw, 0.7rem);
border-bottom: 2px solid var(--border);
font-weight: 600;
font-size: clamp(0.65rem, 0.72vw, 0.78rem);
text-transform: uppercase;
letter-spacing: 0.03em;
color: var(--text-secondary);
white-space: nowrap;
```

### Celulas (`<td>`)

```css
padding: clamp(0.25rem, 0.5vh, 0.4rem) clamp(0.4rem, 0.8vw, 0.7rem);
border-bottom: 1px solid var(--border);
font-variant-numeric: tabular-nums;
font-size: clamp(0.8rem, 0.9vw, 0.95rem);
```

### Zebra striping

```tsx
backgroundColor: index % 2 === 0 ? "transparent" : "var(--bg)"
```

### Celulas numericas

```tsx
textAlign: "right"
fontVariantNumeric: "tabular-nums"
```

### Animacao de linhas

Cada `<tr>` e um `motion.tr` com stagger:

```tsx
<motion.tr
  initial={{ opacity: 0, x: -8 }}
  animate={{ opacity: 1, x: 0 }}
  transition={{ delay: index * 0.03, duration: 0.2 }}
>
```

---

## 10. Graficos SVG (Donuts)

### Donut grande (MarketContextPanel)

```
Viewbox: 150 x 150
Stroke width: 18
Raio: (150 - 18) / 2 = 66
Circunferencia: 2 * PI * 66 = ~414.69
```

- Circulo de fundo: `stroke="var(--border)"`
- Circulo do fornecedor: `stroke="var(--primary)"`, `stroke-linecap="round"`
- Rotacao: `transform="rotate(-90 75 75)"` (inicia no topo)
- Animacao: `strokeDasharray` de `"0 C"` para `"supplierArc (C - supplierArc)"`
- Texto central: percentual em bold + "dos votos" abaixo

Container responsivo:
```tsx
width: "clamp(100px, 12vw, 150px)"
aspectRatio: "1"
```

### Mini donuts (MetricsTable)

```
Viewbox: 80 x 80
Stroke width: 10
Raio: (80 - 10) / 2 = 35
Circunferencia: 2 * PI * 35 = ~219.91
```

- Mesma logica do grande, mas menor
- Texto central: percentual em clamp font

Container responsivo:
```tsx
width: "clamp(52px, 4.8vw, 72px)"
aspectRatio: "1"
```

### Card de mini donut

```tsx
{
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: "clamp(0.2rem, 0.35vh, 0.3rem)",
  padding: "clamp(0.4rem, 0.6vh, 0.55rem) clamp(0.35rem, 0.6vw, 0.5rem)",
  backgroundColor: "var(--bg)",
  borderRadius: "var(--radius-md)",
  border: "1px solid var(--border)",
  minWidth: "clamp(85px, 7.5vw, 115px)",
  flex: "1 0 0",
}
```

---

## 11. Barras de progresso

### Barra de distribuicao (MetricsTable)

**Container:**
```tsx
{
  width: "100%",
  height: "clamp(6px, 0.55vw, 10px)",
  backgroundColor: "var(--border)",
  borderRadius: "5px",
  overflow: "hidden",
}
```

**Fill (animado):**
```tsx
<motion.div
  initial={{ width: 0 }}
  animate={{ width: `${Math.min(pct, 100)}%` }}
  transition={{ delay: index * 0.03 + 0.15, duration: 0.5, ease: "easeOut" }}
  style={{
    height: "100%",
    backgroundColor: "var(--primary)",
    borderRadius: "5px",
  }}
/>
```

### Progress steps (ProcessingScreen)

Cada step e uma row com:

- **Icone circular** (1.75rem x 1.75rem):
  - Completo: fundo `var(--success)` + SVG checkmark branco
  - Ativo: fundo `var(--primary)` + spinner branco (10px, borda 2px, rotate 360)
  - Pendente: fundo `var(--border)` + dot cinza (8px)
- **Background do passo ativo:** `var(--primary-light)`
- **Stagger:** `delay: index * 0.08`

---

## 12. Animacoes e transicoes

### Biblioteca: Framer Motion

Toda animacao usa Framer Motion. **Nao use CSS animations** para elementos interativos, apenas para spinners e pulsos utilitarios.

### Transicao entre telas (ScreenTransition)

```tsx
const variants = {
  enter: { opacity: 0, y: 20 },
  center: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -20 },
};
// Duracao: 0.3s, easeInOut
// Mode: "wait" (tela sai antes da proxima entrar)
```

### Carrossel de metricas (ResultScreen)

```tsx
const carouselVariants = {
  enter: (direction) => ({ x: direction > 0 ? 60 : -60, opacity: 0 }),
  center: { x: 0, opacity: 1 },
  exit: (direction) => ({ x: direction > 0 ? -60 : 60, opacity: 0 }),
};
// Tipo: spring, stiffness 350, damping 32
// Opacidade: 0.18s
// Mode: "popLayout" (sobreposicao fluida)
// Suporta drag (swipe horizontal)
```

### Padroes de entrada de elementos

| Padrao | Uso | Valores |
|--------|-----|---------|
| Fade + slide down | Titulos, headers | `initial: { opacity: 0, y: -10 }`, `duration: 0.4` |
| Fade + slide up | Formularios, botoes | `initial: { opacity: 0, y: 10 }`, `duration: 0.4`, `delay: 0.1` |
| Fade + scale | Icones de status | `initial: { scale: 0.8, opacity: 0 }`, `duration: 0.3` |
| Fade + scale | Dropzone | `initial: { opacity: 0, scale: 0.95 }`, `delay: 0.15` |
| Fade + slide right | Market panel | `initial: { opacity: 0, x: 20 }`, `delay: 0.2` |
| Stagger rows | Linhas de tabela | `delay: index * 0.03`, `initial: { opacity: 0, x: -8 }` |
| Stagger cards | Mini donuts | `delay: index * 0.05 + 0.15`, `initial: { scale: 0.9 }` |

### Interacoes

| Componente | Interacao | Valores |
|-----------|-----------|---------|
| FileDropzone | whileHover | `scale: 1.02`, `borderColor: "var(--primary)"` |
| FileDropzone | whileTap | `scale: 0.98` |
| Upload icon | animate (loop) | `y: [0, -4, 0]`, `duration: 2`, `repeat: Infinity` |
| SegmentedControl indicator | animate | `type: "spring"`, `stiffness: 400`, `damping: 30` |
| btn-primary | CSS hover | `transform: translateY(-1px)` |

### Chrome dimming (ResultScreen)

Quando o carrossel esta transitioning, o header, tab bar e botoes recebem `opacity: 0.45` para focar atencao no conteudo novo:

```tsx
const chromeOpacity = isTransitioning ? 0.45 : 1;
```

### CSS Keyframes (utilities)

Definidos em `globals.css`, usados via classes:

- `.spinner` — `animation: spin 0.8s linear infinite` (para loading generico)
- `.pulse` — `animation: pulse 2s ease-in-out infinite` (para elementos "vivos")
- `.fade-in` — `animation: fadeIn 0.3s ease-out forwards`

---

## 13. Icones (SVG inline)

O app **nao usa icon library** (sem Font Awesome, Lucide, etc). Todos os icones sao SVGs inline no JSX.

### Padroes SVG

```tsx
<svg
  width="SIZE"
  height="SIZE"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"    /* Herda cor do parent */
  strokeWidth="2"
  strokeLinecap="round"
  strokeLinejoin="round"
>
```

### Catalogo de icones usados

| Icone | Tela | SVG (paths) |
|-------|------|-------------|
| Upload | UploadScreen | `<path d="M21 15v4..."/>` + `<polyline points="17 8 12 3 7 8"/>` + `<line x1="12" y1="3" x2="12" y2="15"/>` |
| Documento | UploadScreen (arquivo selecionado) | `<path d="M14 2H6a2 2 0 0 0-2 2v16..."/>` + `<polyline points="14 2 14 8 20 8"/>` |
| Checkmark | ProgressSteps | `<polyline points="20 6 9 17 4 12"/>` |
| X em circulo | ErrorScreen | `<circle cx="12" r="10"/>` + duas linhas cruzadas |
| Chevron left | ResultScreen tabs | `<polyline points="15 18 9 12 15 6"/>` |
| Chevron right | ResultScreen tabs | `<polyline points="9 18 15 12 9 6"/>` |
| Sol | ThemeToggle (dark) | `<circle cx="12" r="5"/>` + 8 linhas radiantes |
| Lua | ThemeToggle (light) | `<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>` |

### Tamanhos de icone

| Contexto | Width/Height |
|----------|-------------|
| Upload icon | 56px |
| Error icon | 32px (dentro de circulo de 64px) |
| File icon | 18px |
| Chevron (tab scroll) | 14px |
| Theme toggle | 16px |
| Check (progress) | 14px |

---

## 14. Telas simples (centralizadas)

As telas Upload, SelectSheet, SelectSupplier, Processing e Error seguem o **mesmo layout base**:

```tsx
<div style={{
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",   /* Centraliza verticalmente */
  gap: "1.5rem",
  textAlign: "center",
  width: "100%",
  flex: 1,                     /* Ocupa todo o espaco vertical */
}}>
```

### Estrutura padrao:

1. **Titulo** — `motion.div` com fade + slide down (`y: -10`)
   - `h1` ou `h2` com peso 700
   - Subtitulo `<p>` com cor `var(--text-secondary)`

2. **Conteudo interativo** — `motion.div` com fade + slide up (`y: 10`, delay 0.1)
   - Select, botoes ou area de interacao
   - `maxWidth: 360px` para formularios

3. **Feedback** (opcional) — Badge de arquivo selecionado, mensagem de erro, etc.

---

## 15. Tela de resultados (complexa)

A ResultScreen **nao se centraliza** — ela preenche toda a viewport com layout flex vertical:

```
┌─────────────────────────────────────┐
│  Header (card com info do supplier) │ flexShrink: 0
├─────────────────────────────────────┤
│  Tab Bar (botoes pill scrollaveis)  │ flexShrink: 0
├──────────┬──────────────────────────┤
│  Market  │  Carousel (metricas)     │ flex: 1, minHeight: 0
│  Context │  com MetricsTable        │
│  Panel   │                          │
├──────────┴──────────────────────────┤
│  Action Buttons (wrap, centered)    │ flexShrink: 0
└─────────────────────────────────────┘
```

### Area de conteudo (panel + carousel)

```tsx
display: "flex",
flexDirection: "row",          /* Lado a lado */
gap: "clamp(0.5rem, 1vw, 1rem)",
flex: 1,
minHeight: 0,
overflow: "hidden",
```

Em mobile (< 700px): `flexDirection: "column"`.

### MarketContextPanel

- Largura fixa (nao cresce): `width: clamp(220px, 22vw, 320px)`
- Em mobile: `width: 100%`, layout horizontal wrapping
- `flexShrink: 0`

### Carousel

- `flex: 1`, `minWidth: 0`, `overflow: hidden`
- Conteudo interno tem `overflow: auto` (scroll se necessario)

---

## 16. Toast de notificacao

O toast e um componente inline no ResultScreen (nao e um componente separado):

```tsx
/* Container */
position: "fixed",
bottom: "6rem",
left: 0, right: 0,
display: "flex",
justifyContent: "center",
zIndex: 100,
pointerEvents: "none",

/* Toast body */
padding: "0.75rem 1.5rem",
backgroundColor: toast.startsWith("Erro") ? "var(--error)" : "var(--success)",
color: "white",
borderRadius: "0.5rem",
fontSize: "0.9rem",
fontWeight: 500,
boxShadow: "0 4px 12px rgba(0,0,0,0.15)",
pointerEvents: "auto",
```

- Animacao: `initial: { opacity: 0, y: 20 }`, `animate: { opacity: 1, y: 0 }`
- Auto-dismiss: `setTimeout(() => setToast(null), 3000)` (sucesso) ou `4000` (erro)

---

## 17. Responsividade

### Breakpoints

| Viewport | Comportamento |
|----------|--------------|
| < 600px | Padding reduzido, fontes menores, botoes menores |
| < 700px | ResultScreen empilha verticalmente (panel acima, carousel abaixo) |
| < 900px | MarketContextPanel largura reduzida |
| 1400px (base) | Max-width do container |
| > 1600px | Container e panel maiores |
| > 2560px | Font-size base 18px |
| > 3840px | Font-size base 20px, container ate 3000px |

### Estrategia de responsividade

1. **Container fluid** com `max-width` progressivo
2. **`clamp()`** para padding, gap, font-size, larguras
3. **Media queries** para mudancas de layout (row -> column)
4. **CSS classes** em vez de media queries inline (`result-screen`, `result-content-area`, `market-context-panel`, `result-carousel`)

### Regra geral para novos componentes

```
padding: clamp(MOBILE, FLUIDO, DESKTOP)
font-size: clamp(MOBILE, FLUIDO, DESKTOP)
gap: clamp(MOBILE, FLUIDO, DESKTOP)
```

Exemplo de formula:
- Mobile: 0.6rem, Fluido: 1.2vh, Desktop: 1rem
- `clamp(0.6rem, 1.2vh, 1rem)`

---

## 18. Scrollbar

### Scrollbar global

```css
::-webkit-scrollbar { width: 6px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb {
  background-color: var(--border-dark);
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background-color: var(--text-secondary);
}
```

### Tab bar (scrollbar escondida)

```css
.tab-bar-scroll {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.tab-bar-scroll::-webkit-scrollbar { display: none; }
```

---

## 19. Checklist para novos componentes

Ao criar um novo componente ou tela, verifique cada item:

### Cores
- [ ] Usa apenas tokens CSS (`var(--primary)`, `var(--surface)`, etc.)
- [ ] Funciona tanto no tema claro quanto no escuro
- [ ] Nao tem cores hardcoded (exceto excecoes documentadas)

### Tipografia
- [ ] Usa `var(--font-family)` (herda do body)
- [ ] Numeros usam `fontVariantNumeric: "tabular-nums"`
- [ ] Font sizes usam `clamp()` para responsividade

### Layout
- [ ] Usa flexbox (nao grid, exceto se necessario)
- [ ] Padding e gap usam `clamp()` para responsividade
- [ ] Se e tela simples: centraliza com `justifyContent: "center"` e `flex: 1`
- [ ] Se e componente complexo: cabe no viewport sem scroll externo

### Cards/Paineis
- [ ] Background: `var(--surface)`
- [ ] Border: `1px solid var(--border)`
- [ ] Border radius: `var(--radius-lg)`
- [ ] Shadow: `var(--shadow-sm)`

### Botoes
- [ ] Usa uma das 3 classes: `btn-primary`, `btn-secondary`, `btn-outline`
- [ ] Tem estado `disabled` com `opacity: 0.5`

### Animacoes
- [ ] Usa Framer Motion (nao CSS transitions para entrada/saida)
- [ ] Segue os padroes de `initial`/`animate` documentados
- [ ] Tem stagger para listas (`delay: index * 0.03`)

### Icones
- [ ] SVG inline (nao icon library)
- [ ] `stroke="currentColor"` para herdar cor
- [ ] `viewBox="0 0 24 24"`, `fill="none"`, `strokeWidth="2"`

### Tabelas
- [ ] Headers uppercase com `var(--text-secondary)`
- [ ] Zebra striping com `var(--bg)`
- [ ] Colunas numericas alinhadas a direita

### Responsividade
- [ ] Testado em 600px, 1024px, 1600px, 2560px
- [ ] Se tem layout lado-a-lado: empilha em mobile (< 700px)
- [ ] Texto nao transborda (word-break, overflow hidden, ellipsis)

### Tema escuro
- [ ] Todas as cores vem de tokens (automaticamente adaptam)
- [ ] Nao tem `background-color: #ffffff` ou similar hardcoded
- [ ] SVGs usam `stroke="currentColor"` ou `var(--token)`
