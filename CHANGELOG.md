# Changelog

Todos los cambios notables de este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [5.8.0] - 2024-09-24

### Agregado
- 🗄️ **Sistema completo de integración con bases de datos**
  - Soporte para MongoDB Atlas (nube) y MongoDB local
  - Almacenamiento automático de servidores encontrados
  - Estructura de datos completa con timestamps
  - Consultas y estadísticas en tiempo real
  - Feature flag `mongodb` para compilación opcional
- 📚 Módulo `database.rs` con cliente genérico
- 📝 Documentación completa DATABASE.md con guía paso a paso
- 🧪 Ejemplo `database_demo.rs` 
- ⚙️ Archivo de configuración `config/database.toml`
- 📊 Estadísticas de BD durante el escaneo
- 🔍 Verificación de duplicados en BD

### Mejorado
- 📦 Sistema modular permite añadir más proveedores fácilmente
- 🔐 Configuración segura con strings de conexión
- ⚡ Guardado asíncrono sin afectar performance del scanner
- 📖 README actualizado con instrucciones de compilación

### Técnico
- Feature flags para compilación condicional
- Dependencia opcional `mongodb = "2.8"`
- Soporte para `chrono` con serialización
- Integración completa en el loop principal del scanner

## [5.7.0] - 2024-09-24

### Agregado
- 🔄 Sistema completo de auto-actualización integrado en el binario
  - Verificación automática al iniciar el programa
  - Comandos `--check-updates` y `--update`
  - Descarga e instalación desde GitHub sin scripts externos
  - Soporte para git pull y descarga directa
  - Recompilación automática después de actualizar
  - Prompt interactivo para confirmar actualizaciones
- 📚 Módulo `updater.rs` con funcionalidad completa
- 📝 Documentación UPDATE.md con guía completa
- 🧪 Ejemplo `updater_demo.rs` demostrando el uso
- 📦 Dependencia `reqwest` para peticiones HTTP

### Corregido
- 🐛 **CRÍTICO**: Modelo de concurrencia del scanner
  - Reemplazado semáforo inefectivo por FuturesUnordered
  - Ahora mantiene pool fijo de exactamente N tareas concurrentes
  - Previene sobrecarga de recursos
- 🐛 **CRÍTICO**: DNS lookup bloqueante
  - Envuelto en `spawn_blocking` para no bloquear runtime de Tokio
  - Mejora significativa en throughput del escaneo
- 🐛 Cálculo impreciso de PPS (packets per second)
  - Usa tiempo real elapsed en lugar de asumir 500ms
  - Métricas ahora son precisas y confiables
- 🐛 Buffer de lectura pequeño (8KB → 16KB)
  - Maneja correctamente MOTDs largos y respuestas grandes
  - Mejorado parsing de JSON para encontrar correctamente inicio/fin

### Eliminado
- 🗑️ Archivos obsoletos y duplicados
  - `src/main.rs` (versión antigua standalone)
  - `copenheimer_oxide/src/main_new.rs` (duplicado)
  - `OmniCracker.exe` (sin documentación, 6MB)
  - Directorio `src/` vacío

### Mejorado
- ⚡ Performance general del scanner con concurrencia real
- 📊 Precisión de métricas y estadísticas
- 🔒 Robustez en manejo de respuestas de servidores
- 📖 Documentación del README con sistema de actualización

## [5.6.0] - 2024-09-24

### Agregado
- Estructura modular del código (config, cidr, server, ui, export, scanner)
- Sistema de configuración externa mediante TOML
- Exportación a múltiples formatos (TXT, JSON, CSV)
- Tests unitarios para módulos principales
- Documentación inline con comentarios rustdoc
- Sistema de logging estructurado con tracing
- Manejo de errores mejorado con Result/thiserror
- README completo con documentación
- LICENSE MIT
- .gitignore apropiado para Rust
- Archivo CHANGELOG.md

### Cambiado
- Refactorización completa del código en módulos separados
- Mejora en el parsing de MOTD
- Optimización de timeouts configurables
- Mejora en la interfaz de usuario

### Mejorado
- Rendimiento del escaneo con mejor gestión de semáforos
- Manejo de errores con menos unwrap()
- Documentación del código

## [5.5.0] - 2024-08-15

### Agregado
- Modo de verificación de servidor único (--check)
- Soporte para intensidades HOME, PRO, NITRO
- Menú interactivo mejorado

### Cambiado
- Timeout de conexión reducido a 150ms para mayor velocidad
- Buffer de lectura aumentado a 8192 bytes

## [5.0.0] - 2024-07-01

### Agregado
- Motor OXIDE completamente reescrito en Rust
- Soporte para hasta 400,000 workers concurrentes
- Sistema de DNS lookup automático
- Múltiples modos de escaneo (Quick, Target, Filter, Deep)
- Rangos CIDR predefinidos de proveedores populares

### Cambiado
- Migración completa de C++ a Rust
- Mejora significativa en rendimiento y estabilidad

## [4.5.0] - 2024-05-20

### Agregado
- Modo Deep Scan con información detallada
- Filtros por jugadores y país
- Contador PPS (paquetes por segundo)

### Mejorado
- Interfaz visual con colored
- Formato de salida más legible

## [4.0.0] - 2024-04-10

### Agregado
- Versión en C++ (ULTRANÍTRO)
- Soporte para 65,000 workers
- Script de instalación para Termux

### Cambiado
- Optimización de protocolo Minecraft
- Mejora en parsing de JSON

## [3.0.0] - 2024-02-15

### Agregado
- Primer prototipo funcional
- Escaneo básico de rangos CIDR
- Guardado de resultados en archivo

### Conocido
- Limitado a 10,000 conexiones concurrentes
- Solo formato TXT de salida

---

## Formato de versionado

- **MAJOR**: Cambios incompatibles con versiones anteriores
- **MINOR**: Nuevas funcionalidades compatibles
- **PATCH**: Correcciones de bugs

[5.6.0]: https://github.com/juancartek/1ueloke/releases/tag/v5.6.0
[5.5.0]: https://github.com/juancartek/1ueloke/releases/tag/v5.5.0
[5.0.0]: https://github.com/juancartek/1ueloke/releases/tag/v5.0.0
[4.5.0]: https://github.com/juancartek/1ueloke/releases/tag/v4.5.0
[4.0.0]: https://github.com/juancartek/1ueloke/releases/tag/v4.0.0
[3.0.0]: https://github.com/juancartek/1ueloke/releases/tag/v3.0.0
