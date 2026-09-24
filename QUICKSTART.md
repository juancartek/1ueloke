# ⚡ Quick Start - COPENHEIMER

¡Empieza a usar COPENHEIMER en 3 minutos! 🚀

---

## 🎯 TL;DR - Para los impacientes

```bash
# Un comando para instalar y ejecutar
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source $HOME/.cargo/env && \
git clone https://github.com/juancartek/1ueloke.git && \
cd 1ueloke/copenheimer_oxide && \
cargo run --release
```

---

## 📦 Instalación (3 pasos)

### 1️⃣ Instalar Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2️⃣ Clonar el proyecto
```bash
git clone https://github.com/juancartek/1ueloke.git
cd 1ueloke/copenheimer_oxide
```

### 3️⃣ Compilar y ejecutar
```bash
cargo run --release
```

**¡Listo!** 🎉

---

## 🎮 Uso Básico

### Modo interactivo
```bash
cargo run --release
```

1. Elige **[1]** Quick Scan (búsqueda ilimitada)
2. Elige **[1]** HOME (5K workers - seguro para casa)
3. ¡Espera a que encuentre servidores!
4. Presiona **Ctrl+C** cuando quieras parar

### Verificar un servidor
```bash
cargo run --release -- --check 51.210.45.123
```

---

## 📊 Ver Resultados

```bash
# Ver archivo de resultados
cat hits_omega.txt

# Ver en tiempo real
tail -f hits_omega.txt
```

**Formato:**
```
🔥 IP: 51.210.45.123 | DOMAIN: mc.server.com | HOST: OVH | VER: 1.20.1 | PLAYERS: 15/100 | MOTD: Survival
```

---

## ⚙️ Configuración Rápida

Edita `config/default.toml`:

```toml
[output]
format = "txt"  # Cambiar a "json" o "csv"
file = "hits_omega.txt"

[intensity]
home = 5000    # Workers para uso doméstico
pro = 25000    # Workers para fibra óptica
nitro = 400000 # Workers para VPS
```

---

## 🆘 Ayuda Rápida

| Problema | Solución |
|----------|----------|
| `cargo: command not found` | `source $HOME/.cargo/env` |
| `too many open files` | `ulimit -n 65000` |
| Escaneo lento | Usa `cargo run --release` (no olvides `--release`) |
| No encuentra servidores | Prueba con intensidad más alta |

---

## 📚 Más Información

- 📖 [Tutorial completo](TUTORIAL.md) - Guía paso a paso
- 🛠️ [Instalación detallada](INSTALL.md) - Solución de problemas
- 🤝 [Contribuir](CONTRIBUTING.md) - Cómo ayudar al proyecto

---

## 💡 Comandos Útiles

```bash
# Ejecutar
cargo run --release

# Verificar IP
cargo run --release -- --check 192.168.1.100

# Ejecutar tests
cargo test

# Ver documentación
cargo doc --open

# Ejecutar ejemplo
cargo run --example basic_scan

# Instalar globalmente
cargo install --path .
copenheimer_oxide  # Ejecutar desde cualquier lugar
```

---

## 🎯 Intensidades Recomendadas

| Conexión | Workers | Comando |
|----------|---------|---------|
| 🏠 Casa (ADSL/Cable) | 5,000 | Elige opción **[1]** |
| 🚀 Fibra (100+ Mbps) | 25,000 | Elige opción **[2]** |
| 🔥 VPS/Dedicado | 400,000 | Elige opción **[3]** |

---

## ⏱️ Tiempos Estimados

| Acción | Tiempo |
|--------|--------|
| Instalar Rust | ~2 minutos |
| Compilar proyecto | ~3-5 minutos |
| Encontrar 10 servidores | ~1-5 minutos |
| Encontrar 100 servidores | ~10-30 minutos |

*Los tiempos varían según tu conexión y hardware*

---

¡Ahora ya sabes lo esencial! 🔥

Para más detalles, revisa el [TUTORIAL.md](TUTORIAL.md) completo.
