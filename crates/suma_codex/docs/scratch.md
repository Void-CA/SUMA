Aquí está tu documento en formato Markdown:

# 🗺️ Fase 1: El Cerebro (suma_core)
Antes de inventar el lenguaje, asegúrate de que la matemática funciona.

- **Crear Lógica:** Crea el módulo en suma_core/src/tudominio.

- **Exponer API:**
Asegúrate de que tus structs (Matrix, Graph, etc.) y funciones sean pub.

- **Testear:**
Escribe tests unitarios en el Core. Si no funciona aquí, no funcionará en Codex.

# 🗺️ Fase 2: El Lenguaje (suma_codex/src/domains)
Definir cómo escribirá el usuario.

- **Andamiaje:**
Crea la carpeta src/domains/NUEVO_DOMINIO/.

- **Archivos:**
mod.rs, grammar.pest, ast.rs, parser.rs.

- **Gramática** (grammar.pest):
- Vital: Copia las reglas WHITESPACE y COMMENT al inicio.
Define tu sintaxis.

- **AST** (ast.rs):
Crea tus structs/enums. ⚠️ Vital: Deben derivar #[derive(Debug, Clone, Serialize)].

- **Parser** (parser.rs):
Implementa el trait DomainParser. Traduce los Pairs de Pest a tus structs del AST.

- **Registro:**
Agrega pub mod nuevo_dominio; en src/domains/mod.rs.

# 🗺️ Fase 3: El Cableado (suma_codex/src/engine y ast)
Conectar el nuevo dominio al sistema central. Aquí es donde suele olvidarse algo.

## Result Global (src/ast/mod.rs)
Agrega tu modelo al enum CodexResult. NewDomain(NewDomainModel),

## Dispatcher (src/engine/dispatcher.rs)
Import: use crate::domains::nuevo_dominio::ast::NewDomainModel; Handle: En handle_domain_block, agrega el else if let Some(...) para inyectar el nombre (.name). Convert: En convert_and_store, agrega el else if para pushear al CodexResult.

# 🗺️ Fase 4: La Ejecución (suma_codex/src/engine/adapters)
Hacer que las cosas sucedan.

## Adaptador (src/engine/adapters/nuevo_dominio.rs)
Crea el struct NewDomainExecutor. Implementa execute(&self, model). Aquí importas suma_core y traduces el AST a llamadas reales. Registro de Adaptador: Agrega pub mod nuevo_dominio; en src/engine/adapters/mod.rs. Executor Global (src/engine/executor.rs) En el match result, agrega el caso para CodexResult::NewDomain. Instancia tu adaptador y llama a execute.

## Checklist de Errores Comunes (Tus lecciones aprendidas)
"Panic: expected factor" en comentarios: ¿Pusiste la regla COMMENT en grammar.pest? ¿Tu regla de división (/) tiene el hack ("/" ≠ !"/") para no comerse los comentarios? "El test pasa pero no ejecuta nada": ¿Agregaste el else if en dispatcher.rs -> convert_and_store? Si no lo haces, el parser funciona pero tira el resultado a la basura. "Trait Bound not satisfied": ¿Tu AST tiene Clone y Serialize? "Method not found in Core": ¿Hiciste pub las funciones en suma_core? Tests silenciosos: Usa siempre cargo test -- --nocapture cuando debuggeas parsers.
