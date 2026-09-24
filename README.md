# 🔥 COPENHEIMER - Minecraft Server Scanner

<div align="center">

```
   ____  _____  ____  ____  _      _      ____  _____  ____  ____  ____ 
  /  _ \/  __/ /  _ \/  _ \/ \  /|/ \__/|/  _ \/__ __\/  _ \/  _ \/  _ \
  | / \||  \   | / \|| | //| |\ ||| |\/||| / \|  / \  | | //| / \|| | //
  | \_/||  /_  | \_/|| |_\\| | \||| |  ||| \_/|  | |  | |_\\| \_/|| |_\\
  \____/\____\ \____/\____/\_/  \|\_/  \|\____/  \_/  \____/\____/\____/
```

**Ultra-fast Minecraft server scanner written in Rust 🦀**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-5.8.0-blue.svg)](https://github.com/juancartek/1ueloke)

</div>

---

## 📖 Documentación

| Guía | Descripción | Tiempo |
|------|-------------|--------|
| **[⚡ QUICKSTART](QUICKSTART.md)** | Empieza en 3 minutos | 3 min |
| **[🛠️ INSTALL](INSTALL.md)** | Instalación completa y solución de problemas | 10 min |
| **[🎓 TUTORIAL](TUTORIAL.md)** | Tutorial paso a paso con ejemplos | 20 min |
| **[📊 DATABASE](DATABASE.md)** | Integración con MongoDB Atlas y otras BD | 15 min |
| **[🤝 CONTRIBUTING](CONTRIBUTING.md)** | Cómo contribuir al proyecto | 15 min |
| **[📝 CHANGELOG](CHANGELOG.md)** | Historial de versiones | 5 min |

---

## 📋 Descripción

**COPENHEIMER** es un escáner masivo de servidores de Minecraft diseñado para descubrir servidores activos mediante el escaneo eficiente de rangos CIDR. Construido con Rust y Tokio, puede manejar hasta **400,000 conexiones concurrentes** para lograr velocidades de escaneo extremadamente altas.

### ✨ Características principales

- 🚀 **Ultra rápido**: Hasta 400K tareas concurrentes con Tokio
- 🎯 **Modos de escaneo**: Quick Scan, Target Mode, Filter Mode y Deep Scan
- 🌍 **DNS Lookup**: Resolución automática de dominios
- 📊 **Múltiples formatos de exportación**: TXT, JSON, CSV
- 🗄️ **Integración con bases de datos**: MongoDB Atlas, PostgreSQL, MySQL, SQLite
- 🎨 **Interface colorida**: Terminal UI atractiva con colored
- ⚙️ **Configurable**: Archivo TOML para personalización
- 🔍 **Filtros avanzados**: Por jugadores, país, versión
- 💾 **Persistencia**: Guarda resultados automáticamente
- 🔄 **Auto-actualización**: Sistema integrado de actualizaciones automáticas

---

## 🚀 Instalación

### 📋 Requisitos previos

- **Rust 1.70+** ([Instalar Rust](https://rustup.rs/))
- Sistema operativo: Linux, Windows, macOS, Termux

### ⚡ Instalación Rápida

```bash
# 1. Instalar Rust (si no lo tienes)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clonar el repositorio
git clone https://github.com/juancartek/1ueloke.git
cd 1ueloke/copenheimer_oxide

# 3. Compilar en modo release (optimizado)
cargo build --release

# Con soporte para MongoDB Atlas
cargo build --release --features mongodb

# 4. Ejecutar
./target/release/copenheimer_oxide

# O ejecutar directamente con Cargo
cargo run --release
```

> 📚 **[Guía completa de instalación →](INSTALL.md)** con solución de problemas y comandos útiles

### Instalación en Termux (Android)

```bash
# Descargar y ejecutar el script de instalación
curl -L https://raw.githubusercontent.com/juancartek/1ueloke/main/copenheimer_termux.sh | bash
```

### Binarios precompilados

Descarga los binarios desde [Releases](https://github.com/juancartek/1ueloke/releases):

- `copenheimer_linux` - Para Linux x86_64
- `copenheimer_windows.exe` - Para Windows x86_64

---

## 📖 Uso

### Modo básico

```bash
# Ejecutar con menú interactivo
./copenheimer_oxide

# Verificar un servidor específico
./copenheimer_oxide --check 192.168.1.100

# Verificar actualizaciones
./copenheimer_oxide --check-updates

# Actualizar a la última versión
./copenheimer_oxide --update
```

### Actualización Automática 🔄

COPENHEIMER incluye un sistema de auto-actualización integrado:

- **Al iniciar el programa**: Verifica automáticamente si hay actualizaciones y te pregunta si quieres instalarlas
- **Manual**: Usa `./copenheimer_oxide --update` para actualizar en cualquier momento
- **Sin scripts externos**: Todo está integrado en el binario, no necesitas ejecutar scripts bash

```bash
# El programa detecta actualizaciones automáticamente:
./copenheimer_oxide

# Salida:
# ═══════════════════════════════════════════════════
#    ✨ NUEVA ACTUALIZACIÓN DISPONIBLE ✨
# ═══════════════════════════════════════════════════
#    Versión: v1.1.0 → v1.2.0
# ═══════════════════════════════════════════════════
#
#    ¿Deseas actualizar ahora? (s/n): s
#
# 🔄 Descargando...
# ✅ Actualización completada
```

### Configuración

Edita `config/default.toml` para personalizar:

```toml
[scan]
timeout_ms = 300          # Timeout de conexión
workers = 65000           # Número de workers concurrentes
port = 25565              # Puerto de Minecraft

[output]
format = "txt"            # txt, json, csv
file = "hits_omega.txt"   # Archivo de salida
```

### Modos de escaneo

#### 1️⃣ Quick Scan - Búsqueda ilimitada
Escanea continuamente hasta que lo detengas manualmente.

```
Opción: 1
```

#### 2️⃣ Target Mode - Cantidad específica
Para automáticamente después de encontrar N servidores.

```
Opción: 2
Cantidad deseada: 100
```

#### 3️⃣ Filter Mode - Filtros avanzados
Filtra por cantidad de jugadores y país.

```
Opción: 3
Min jugadores: 5
País: US
```

#### 4️⃣ Deep Scan - Información detallada
Muestra información completa de cada servidor encontrado.

```
Opción: 4
```

### Intensidades

- **HOME** (5,000 workers) - Para conexiones domésticas
- **PRO** (25,000 workers) - Para fibra óptica
- **NITRO** (400,000 workers) - Para VPS/servidores dedicados

---

## 📁 Estructura del proyecto

```
copenheimer/
├── Cargo.toml                 # Configuración de Rust
├── config/
│   └── default.toml          # Configuración por defecto
├── src/
│   ├── main.rs               # Punto de entrada
│   ├── scanner.rs            # Lógica de escaneo
│   ├── cidr.rs               # Gestión de rangos CIDR
│   ├── server.rs             # Parsing de respuestas Minecraft
│   ├── ui.rs                 # Interfaz de usuario
│   ├── config.rs             # Carga de configuración
│   └── export.rs             # Exportación a formatos
├── tests/                    # Tests unitarios
└── examples/                 # Ejemplos de uso
```

---

## 🎯 Ejemplos de salida

### Servidor encontrado

```
   ╭──────────────────────────────────────────────────────────────────────────╮
   🔥  IP: 51.210.45.123   🏢  HOST: OVH/Gaming              
   🛠️   VER: 1.20.1         👥  PLAYERS: 15/100
   📝  MOTD: Survival Server - Join Now!                      
   ╰──────────────────────────────────────────────────────────────────────────╯
```

### Formato de archivo de salida (hits_omega.txt)

```
🔥 IP: 51.210.45.123 | DOMAIN: mc.example.com | HOST: OVH/Gaming | VER: 1.20.1 | PLAYERS: 15/100 | MOTD: Survival Server
```

---

## ⚠️ Advertencias y uso responsable

> **IMPORTANTE**: Esta herramienta es solo para fines educativos y de investigación.

- ✅ **Uso legítimo**: Investigación de seguridad, análisis de infraestructura
- ❌ **Prohibido**: Ataques DDoS, sobrecarga intencional, usos maliciosos
- 🔒 **Responsabilidad**: Respeta los términos de servicio de los proveedores
- 🌐 **Rate limiting**: Usa intensidades bajas en redes compartidas
- 📜 **Legal**: Asegúrate de cumplir con las leyes locales

**El autor no se hace responsable del mal uso de esta herramienta.**

---

## 🛠️ Desarrollo

### Ejecutar tests

```bash
cargo test
```

### Generar documentación

```bash
cargo doc --open
```

### Formatear código

```bash
cargo fmt
```

### Linter

```bash
cargo clippy
```

---

## 🤝 Contribuciones

Las contribuciones son bienvenidas! Por favor:

1. Haz fork del proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-caracteristica`)
3. Commit tus cambios (`git commit -m 'Agregar nueva característica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Abre un Pull Request

---

## 📝 Changelog

Ver [CHANGELOG.md](CHANGELOG.md) para el historial completo de cambios.

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

---

## 👨‍💻 Autor

**juancartek**

- GitHub: [@juancartek](https://github.com/juancartek)
- Proyecto: [1ueloke](https://github.com/juancartek/1ueloke)

---

## 🙏 Agradecimientos

- [Tokio](https://tokio.rs/) - Runtime asíncrono
- [Colored](https://github.com/mackwic/colored) - Terminal colors
- Comunidad de Rust 🦀

---

<div align="center">

**⭐ Si te gusta este proyecto, dale una estrella en GitHub! ⭐**

</div>
