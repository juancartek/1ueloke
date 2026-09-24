# Guía de Contribución

¡Gracias por tu interés en contribuir a COPENHEIMER! Este documento te guiará en el proceso.

## 🚀 Inicio Rápido

### Requisitos

- Rust 1.70 o superior
- Git
- Editor de código (recomendado: VSCode con rust-analyzer)

### Configurar el entorno

```bash
# Clonar el repositorio
git clone https://github.com/juancartek/1ueloke.git
cd 1ueloke/copenheimer_oxide

# Compilar el proyecto
cargo build

# Ejecutar tests
cargo test

# Ejecutar el programa
cargo run
```

## 📝 Proceso de Contribución

### 1. Fork y Clone

1. Haz fork del repositorio en GitHub
2. Clona tu fork localmente
3. Configura el repositorio upstream

```bash
git remote add upstream https://github.com/juancartek/1ueloke.git
```

### 2. Crear una rama

```bash
git checkout -b feature/mi-nueva-caracteristica
```

Usa nombres descriptivos:
- `feature/` - Para nuevas características
- `fix/` - Para correcciones de bugs
- `docs/` - Para cambios en documentación
- `refactor/` - Para refactorización de código

### 3. Hacer cambios

- Sigue las convenciones de código de Rust
- Agrega tests para nuevas características
- Actualiza la documentación si es necesario
- Ejecuta `cargo fmt` antes de commitear
- Ejecuta `cargo clippy` para detectar problemas

### 4. Commit

Usa mensajes de commit descriptivos:

```bash
git commit -m "feat: agregar soporte para filtro por versión"
git commit -m "fix: corregir timeout en conexiones lentas"
git commit -m "docs: actualizar README con nuevos ejemplos"
```

### 5. Push y Pull Request

```bash
git push origin feature/mi-nueva-caracteristica
```

Luego crea un Pull Request en GitHub con:
- Descripción clara de los cambios
- Referencias a issues relacionados
- Screenshots si aplica

## 🧪 Tests

### Ejecutar todos los tests

```bash
cargo test
```

### Ejecutar tests específicos

```bash
# Tests de un módulo específico
cargo test cidr

# Tests de integración
cargo test --test integration_tests
```

### Agregar nuevos tests

Los tests deben estar en:
- `src/` - Tests unitarios junto al código (`#[cfg(test)] mod tests`)
- `tests/` - Tests de integración

Ejemplo:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mi_funcion() {
        let resultado = mi_funcion(42);
        assert_eq!(resultado, esperado);
    }
}
```

## 📚 Documentación

### Documentar código

Usa comentarios de documentación:

```rust
/// Descripción breve de la función
///
/// Descripción más detallada si es necesario.
///
/// # Argumentos
///
/// * `param` - Descripción del parámetro
///
/// # Ejemplo
///
/// ```
/// use copenheimer_oxide::modulo::funcion;
/// let resultado = funcion(valor);
/// ```
pub fn funcion(param: Tipo) -> Resultado {
    // implementación
}
```

### Generar documentación

```bash
cargo doc --open
```

## 🎨 Estilo de Código

### Formato

Usa `rustfmt`:

```bash
cargo fmt
```

### Linting

Usa `clippy` para detectar problemas:

```bash
cargo clippy
```

### Convenciones

- Nombres de variables: `snake_case`
- Nombres de funciones: `snake_case`
- Nombres de tipos: `PascalCase`
- Nombres de constantes: `UPPER_SNAKE_CASE`
- Límite de línea: 100 caracteres (flexible)

## 🐛 Reportar Bugs

Cuando reportes un bug, incluye:

1. **Versión** de Rust y del programa
2. **Sistema operativo** y versión
3. **Pasos para reproducir** el problema
4. **Comportamiento esperado** vs **comportamiento actual**
5. **Logs o mensajes de error** si aplica

## ✨ Sugerir Características

Para sugerir nuevas características:

1. Verifica que no exista ya un issue similar
2. Describe claramente el problema que resuelve
3. Proporciona ejemplos de uso
4. Discute alternativas consideradas

## 📋 Checklist antes del PR

- [ ] El código compila sin errores (`cargo build`)
- [ ] Todos los tests pasan (`cargo test`)
- [ ] El código está formateado (`cargo fmt`)
- [ ] No hay warnings de clippy (`cargo clippy`)
- [ ] Se agregaron tests para nuevas características
- [ ] Se actualizó la documentación si es necesario
- [ ] El commit sigue las convenciones
- [ ] El PR tiene una descripción clara

## 🤝 Código de Conducta

- Sé respetuoso y constructivo
- Acepta críticas constructivas
- Enfócate en lo mejor para el proyecto
- Ayuda a otros contributors

## 📞 Contacto

- GitHub Issues: Para bugs y features
- Discussions: Para preguntas y discusiones generales

---

¡Gracias por contribuir a COPENHEIMER! 🔥
