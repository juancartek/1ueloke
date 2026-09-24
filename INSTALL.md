# 📦 Guía de Instalación y Uso - COPENHEIMER

Esta guía te llevará paso a paso desde la instalación de Rust hasta ejecutar COPENHEIMER.

---

## 🦀 Paso 1: Instalar Rust

### En Linux / macOS / WSL

```bash
# Descargar e instalar Rust usando rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Seleccionar la opción 1 (instalación por defecto)
# Después, cargar el entorno
source $HOME/.cargo/env

# Verificar instalación
rustc --version
cargo --version
```

### En Windows

1. Descarga [rustup-init.exe](https://win.rustup.rs/)
2. Ejecuta el instalador
3. Sigue las instrucciones (instalación por defecto)
4. Abre una nueva terminal CMD o PowerShell
5. Verifica: `rustc --version`

### En Termux (Android)

```bash
pkg update && pkg upgrade
pkg install rust git
```

---

## 📥 Paso 2: Descargar el Proyecto

```bash
# Clonar el repositorio
git clone https://github.com/juancartek/1ueloke.git

# Entrar al directorio
cd 1ueloke/copenheimer_oxide

# Ver la estructura
ls -la
```

---

## 🔨 Paso 3: Compilar el Proyecto

### Compilación en modo desarrollo (más rápido)

```bash
cargo build
```

Esto genera el binario en: `target/debug/copenheimer_oxide`

### Compilación en modo release (optimizado)

```bash
cargo build --release
```

Esto genera el binario en: `target/release/copenheimer_oxide`

**Recomendado:** Siempre usar `--release` para uso real, es mucho más rápido.

---

## ▶️ Paso 4: Ejecutar el Programa

### Opción A: Ejecutar directamente con Cargo

```bash
# Modo desarrollo
cargo run

# Modo release (RECOMENDADO)
cargo run --release
```

### Opción B: Ejecutar el binario directamente

```bash
# Linux / macOS / Termux
./target/release/copenheimer_oxide

# Windows
.\target\release\copenheimer_oxide.exe
```

### Opción C: Instalar en el sistema

```bash
# Instala el binario en ~/.cargo/bin/
cargo install --path .

# Ahora puedes ejecutarlo desde cualquier lugar
copenheimer_oxide
```

---

## 🎯 Paso 5: Usar el Programa

### Modo interactivo (por defecto)

```bash
cargo run --release
```

Verás el menú:

```
   ╔══════════════════════════════════════════════════════════════════════════╗
   ║                  COPENHEIMER ULTRA-NITRO v5.6.0                          ║
   ╚══════════════════════════════════════════════════════════════════════════╝

   🎮 Selecciona una opción:
   [1] 🚀 Quick Scan  -  Búsqueda ilimitada
   [2] 🎯 Target Mode  -  Buscar N servidores y parar

   👉 Opción >
```

1. **Elige el modo** (1 o 2)
2. Si eliges 2, indica cuántos servidores quieres encontrar
3. **Elige la intensidad:**
   - [1] HOME - 5,000 workers (para casa)
   - [2] PRO - 25,000 workers (fibra óptica)
   - [3] NITRO - 400,000 workers (VPS/servidores)

### Modo check (verificar un servidor específico)

```bash
# Verificar si una IP tiene servidor Minecraft
cargo run --release -- --check 51.210.45.123

# O con el binario instalado
copenheimer_oxide --check 192.168.1.100
```

---

## ⚙️ Paso 6: Configurar (Opcional)

Edita el archivo `config/default.toml`:

```toml
[scan]
timeout_ms = 300          # Timeout de conexión
workers = 65000           # Workers por defecto
port = 25565              # Puerto de Minecraft

[output]
format = "txt"            # Formato: txt, json, csv
file = "hits_omega.txt"   # Archivo de salida

[cidr_ranges]
# Agrega o quita rangos CIDR
ranges = [
    "51.0.0.0/8:OVH/Gaming:EU",
    "144.76.0.0/16:Hetzner DE:DE",
    # ... más rangos
]
```

---

## 📊 Resultados

Los servidores encontrados se guardan en `hits_omega.txt`:

```
🔥 IP: 51.210.45.123 | DOMAIN: mc.example.com | HOST: OVH/Gaming | VER: 1.20.1 | PLAYERS: 15/100 | MOTD: Welcome!
```

Para exportar en JSON o CSV, cambia `format = "json"` o `format = "csv"` en el config.

---

## 🧪 Comandos Útiles

### Ejecutar tests

```bash
cargo test
```

### Ver documentación

```bash
cargo doc --open
```

### Ejecutar ejemplos

```bash
# Ejemplo de escaneo básico
cargo run --example basic_scan

# Verificar un servidor
cargo run --example check_server 51.210.45.123

# Demo de exportación
cargo run --example export_demo
```

### Limpiar archivos compilados

```bash
cargo clean
```

---

## 🚀 Instalación Rápida (Un Comando)

### Linux / macOS / WSL

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source $HOME/.cargo/env && \
git clone https://github.com/juancartek/1ueloke.git && \
cd 1ueloke/copenheimer_oxide && \
cargo build --release && \
./target/release/copenheimer_oxide
```

### Termux (Android)

```bash
pkg update && pkg install rust git -y && \
git clone https://github.com/juancartek/1ueloke.git && \
cd 1ueloke/copenheimer_oxide && \
ulimit -n 65000 2>/dev/null && \
cargo run --release
```

---

## 🔧 Solución de Problemas

### Error: "cargo: command not found"

```bash
# Agregar Cargo al PATH
export PATH="$HOME/.cargo/bin:$PATH"

# O agregar permanentemente a ~/.bashrc o ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Error: "linker 'cc' not found"

**Linux:**
```bash
# Debian/Ubuntu
sudo apt install build-essential

# Fedora
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

**macOS:**
```bash
xcode-select --install
```

### Error: "too many open files"

```bash
# Aumentar límite de archivos abiertos
ulimit -n 65000

# O agregar permanentemente
echo "* soft nofile 65000" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65000" | sudo tee -a /etc/security/limits.conf
```

### Compilación lenta

```bash
# Usar compilación paralela
cargo build --release -j $(nproc)

# O establecer en ~/.cargo/config.toml
mkdir -p ~/.cargo
echo -e "[build]\njobs = 4" > ~/.cargo/config.toml
```

---

## 📝 Notas Importantes

### Rendimiento

- **Modo debug**: Lento pero con información de depuración
- **Modo release**: Hasta 10x más rápido ⚡

### Intensidades

| Intensidad | Workers | Uso recomendado |
|------------|---------|-----------------|
| HOME       | 5,000   | Conexión doméstica (ADSL/Cable) |
| PRO        | 25,000  | Fibra óptica residencial |
| NITRO      | 400,000 | VPS/Servidores dedicados |

### Límites del sistema

En sistemas Linux/macOS, puede que necesites aumentar los límites:

```bash
# Ver límite actual
ulimit -n

# Aumentar temporalmente
ulimit -n 65000
```

---

## 🎓 Próximos Pasos

1. ✅ **Explora los ejemplos**: `cargo run --example basic_scan`
2. ✅ **Lee la documentación**: `cargo doc --open`
3. ✅ **Personaliza la config**: Edita `config/default.toml`
4. ✅ **Contribuye**: Lee `CONTRIBUTING.md`

---

## 💬 Soporte

- **Issues**: https://github.com/juancartek/1ueloke/issues
- **Documentación Rust**: https://doc.rust-lang.org/
- **Cargo Book**: https://doc.rust-lang.org/cargo/

---

¡Listo! Ahora tienes COPENHEIMER funcionando 🔥🦀
