# 🎮 Tutorial de Uso - COPENHEIMER

Guía visual paso a paso para usar COPENHEIMER por primera vez.

---

## 📖 Índice

1. [Instalación Rápida](#instalación-rápida)
2. [Primera Ejecución](#primera-ejecución)
3. [Modo Quick Scan](#modo-quick-scan)
4. [Modo Target](#modo-target)
5. [Verificar Servidor Específico](#verificar-servidor-específico)
6. [Ver Resultados](#ver-resultados)
7. [Personalizar Configuración](#personalizar-configuración)

---

## 🚀 Instalación Rápida

### Opción 1: Script Automático (Recomendado)

```bash
curl -sSL https://raw.githubusercontent.com/juancartek/1ueloke/main/install.sh | bash
```

### Opción 2: Manual

```bash
# Instalar Rust si no lo tienes
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clonar y compilar
git clone https://github.com/juancartek/1ueloke.git
cd 1ueloke/copenheimer_oxide
cargo build --release
```

---

## 🎬 Primera Ejecución

```bash
cd 1ueloke/copenheimer_oxide
cargo run --release
```

Verás el logo y el menú principal:

```
   ╔══════════════════════════════════════════════════════════════════════════╗
   ║      __  ____   standard  ____  _   _  _____  ____   ____               ║
   ║     / / / / /  /_  __/ __ \/ | / / /_  __/ __ \/ __ \                   ║
   ║    / / / / /    / / / /_/ /  |/ /   / / / /_/ / / / /                   ║
   ║   / /_/ / /___ / / / _, _/ /|  /   / / / _, _/ /_/ /                    ║
   ║   \____/_____//_/ /_/ |_/_/ |_/   /_/ /_/ |_|\____/                     ║
   ║                                                                          ║
   ║  🚀  COPENHEIMER ULTRA-NITRO v5.6.0 | OXIDE ENGINE                      ║
   ╚══════════════════════════════════════════════════════════════════════════╝

   🎮 Selecciona una opción:
   [1] 🚀 Quick Scan  -  Búsqueda ilimitada
   [2] 🎯 Target Mode  -  Buscar N servidores y parar

   👉 Opción > _
```

---

## 🚀 Modo Quick Scan

**Búsqueda continua sin límite** - Presiona Ctrl+C para detener

### Paso 1: Seleccionar modo
```
👉 Opción > 1
```

### Paso 2: Elegir intensidad
```
   ⚡ Selecciona la Intensidad:
   [1] 🏠 HOME   -  Para routers normales (Seguro)
   [2] 🚀 PRO    -  Para fibra óptica (Rápido)
   [3] 🔥 NITRO  -  Para VPS/Dedicados (Extremo)

   👉 Intensidad > 1
```

**Recomendaciones:**
- **HOME (5K workers)**: Internet doméstico, cable, ADSL
- **PRO (25K workers)**: Fibra óptica en casa
- **NITRO (400K workers)**: Solo en VPS o servidores dedicados

### Paso 3: Ver resultados en tiempo real

```
   🔥 Iniciando motor con 5000 workers...

  🚀 PPS: 2.5K | Scanned: 15.3K | Found: 3 

   ╭──────────────────────────────────────────────────────────────────────────╮
   🔥  IP: 51.210.45.123   🏢  HOST: OVH/Gaming              
   🛠️   VER: 1.20.1         👥  PLAYERS: 15/100
   📝  MOTD: ⚡ Survival Server - Join Now!                      
   ╰──────────────────────────────────────────────────────────────────────────╯
```

**Leyenda:**
- **PPS**: Paquetes por segundo (velocidad de escaneo)
- **Scanned**: Total de IPs verificadas
- **Found**: Servidores encontrados

### Paso 4: Detener el escaneo

Presiona **Ctrl + C** para detener cuando quieras.

---

## 🎯 Modo Target

**Buscar N servidores específicos y parar automáticamente**

### Paso 1: Seleccionar modo
```
👉 Opción > 2
```

### Paso 2: Indicar cantidad deseada
```
   🔢 Cuántos servidores quieres encontrar? > 10
```

### Paso 3: Elegir intensidad
```
   👉 Intensidad > 2
```

### Paso 4: Esperar hasta completar

El escaneo se detendrá automáticamente al alcanzar el objetivo:

```
   ╭──────────────────────────────────────────────────────────────────────────╮
   🔥  IP: 144.76.82.45    🏢  HOST: Hetzner DE              
   🛠️   VER: 1.19.4         👥  PLAYERS: 5/50
   📝  MOTD: Vanilla Survival                      
   ╰──────────────────────────────────────────────────────────────────────────╯

   🏁 Objetivo alcanzado.
   ⏱️  Tiempo total: 2m 34s.
   🔍 IPs verificadas: 234.5K.

   👉 Presiona [ENTER] para volver al menú principal...
```

---

## 🔍 Verificar Servidor Específico

**Comprobar el estado de una IP conocida**

```bash
cargo run --release -- --check 51.210.45.123
```

**Resultado si está online:**
```
   ╭──────────────────────────────────────────────────────────────────────────╮
   🔥  IP: 51.210.45.123   🏢  HOST: mc.example.com              
   🛠️   VER: 1.20.1         👥  PLAYERS: 15/100
   📝  MOTD: Welcome to our server!                      
   ╰──────────────────────────────────────────────────────────────────────────╯
```

**Resultado si está offline:**
```
OFFLINE
```

### Uso con binario instalado

Si instalaste globalmente:
```bash
copenheimer_oxide --check 192.168.1.100
```

---

## 📄 Ver Resultados

Los resultados se guardan automáticamente en `hits_omega.txt`:

```bash
cat hits_omega.txt
```

**Formato de salida:**
```
🔥 IP: 51.210.45.123 | DOMAIN: mc.example.com | HOST: OVH/Gaming | VER: 1.20.1 | PLAYERS: 15/100 | MOTD: Survival Server
🔥 IP: 144.76.82.45 | DOMAIN: play.server.net | HOST: Hetzner DE | VER: 1.19.4 | PLAYERS: 5/50 | MOTD: Vanilla Survival
🔥 IP: 135.148.23.67 | DOMAIN: N/A | HOST: OVH/Sparked | VER: 1.18.2 | PLAYERS: 1/20 | MOTD: Test Server
```

### Ver en tiempo real

```bash
# Linux / macOS
tail -f hits_omega.txt

# Windows PowerShell
Get-Content hits_omega.txt -Wait
```

---

## ⚙️ Personalizar Configuración

Edita `config/default.toml` para personalizar:

### Cambiar formato de salida

```toml
[output]
format = "json"  # txt, json, csv
file = "mis_servidores.json"
```

**Formatos disponibles:**

#### TXT (por defecto)
```
🔥 IP: 51.210.45.123 | DOMAIN: mc.example.com | ...
```

#### JSON
```json
{"ip":"51.210.45.123","domain":"mc.example.com","provider":"OVH","version":"1.20.1","players":{"online":15,"max":100},"motd":"Survival"}
```

#### CSV
```csv
IP,Domain,Provider,Version,Online,Max,MOTD
51.210.45.123,mc.example.com,OVH/Gaming,1.20.1,15,100,Survival Server
```

### Ajustar timeouts

```toml
[scan]
timeout_ms = 300       # Más bajo = más rápido pero menos confiable
read_timeout_ms = 400  # Tiempo para leer respuesta
```

### Agregar/quitar rangos CIDR

```toml
[cidr_ranges]
ranges = [
    "51.0.0.0/8:OVH/Gaming:EU",
    "144.76.0.0/16:Hetzner DE:DE",
    "TU_RANGO_PERSONALIZADO/16:TuProvider:TU",
]
```

### Cambiar intensidades por defecto

```toml
[intensity]
home = 10000   # Cambiar de 5000 a 10000
pro = 50000    # Cambiar de 25000 a 50000
nitro = 500000 # Cambiar de 400000 a 500000
```

---

## 💡 Consejos y Trucos

### 1. Escaneo nocturno programado

```bash
# Linux / macOS con cron
crontab -e
# Agregar: 0 2 * * * cd /ruta/a/copenheimer_oxide && cargo run --release
```

### 2. Guardar logs con timestamp

```bash
cargo run --release 2>&1 | tee "scan_$(date +%Y%m%d_%H%M%S).log"
```

### 3. Filtrar resultados por jugadores

```bash
# Encontrar servidores con más de 10 jugadores
grep "PLAYERS: [1-9][0-9]/" hits_omega.txt
```

### 4. Extraer solo las IPs

```bash
grep -oP 'IP: \K[0-9.]+' hits_omega.txt
```

### 5. Estadísticas rápidas

```bash
# Total de servidores encontrados
wc -l hits_omega.txt

# Servidores por proveedor
grep -oP 'HOST: \K[^|]+' hits_omega.txt | sort | uniq -c | sort -rn

# Versiones más comunes
grep -oP 'VER: \K[^|]+' hits_omega.txt | sort | uniq -c | sort -rn
```

---

## 🐛 Problemas Comunes

### "cargo: command not found"

```bash
source $HOME/.cargo/env
# O agregar a ~/.bashrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```

### "too many open files"

```bash
ulimit -n 65000
```

### Escaneo muy lento

1. Asegúrate de usar `--release`
2. Reduce los workers si tu conexión es lenta
3. Verifica tu conexión a internet

### No encuentra servidores

1. Prueba con intensidad más alta
2. Verifica los rangos CIDR en `config/default.toml`
3. Algunos rangos pueden tener pocos servidores activos

---

## 📚 Más Información

- **Guía de instalación completa**: [INSTALL.md](INSTALL.md)
- **Contribuir al proyecto**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Historial de cambios**: [CHANGELOG.md](CHANGELOG.md)
- **Documentación del código**: `cargo doc --open`

---

## 🎓 Ejercicios Prácticos

### Ejercicio 1: Primer escaneo
1. Ejecuta en modo Quick Scan con intensidad HOME
2. Deja correr 5 minutos
3. Detén con Ctrl+C
4. Revisa cuántos servidores encontró

### Ejercicio 2: Búsqueda objetivo
1. Usa Target Mode
2. Busca exactamente 5 servidores
3. Usa intensidad PRO
4. Anota el tiempo que tardó

### Ejercicio 3: Exportación JSON
1. Cambia el formato a JSON en `config/default.toml`
2. Encuentra 3 servidores
3. Abre el archivo JSON y analiza la estructura

### Ejercicio 4: Verificación
1. Toma una IP del archivo de resultados
2. Verifica su estado con `--check`
3. Compara con el resultado guardado

---

¡Ahora estás listo para usar COPENHEIMER como un profesional! 🔥🦀
