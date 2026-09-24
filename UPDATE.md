# 🔄 Sistema de Auto-Actualización

COPENHEIMER incluye un sistema completo de auto-actualización que verifica y descarga actualizaciones desde GitHub automáticamente.

---

## 📋 Métodos de Actualización

### 1️⃣ Verificar Actualizaciones

Verifica si hay una nueva versión disponible sin instalarla:

```bash
# Usando el binario compilado
./target/release/copenheimer_oxide --check-updates

# O directamente con Cargo
cargo run --release -- --check-updates

# O usando el script bash
./scripts/update_checker.sh
```

**Salida esperada:**
```
🔍 Verificando actualizaciones...

📦 Versión local:  v1.1.0
☁️  Versión remota: v1.2.0

✨ ¡Nueva actualización disponible!
   v1.1.0 → v1.2.0

Para actualizar, ejecuta:
  ./scripts/auto_update.sh
```

---

### 2️⃣ Instalar Actualización

Descarga e instala la última versión automáticamente:

```bash
# Usando el binario compilado
./target/release/copenheimer_oxide --update

# O usando el script bash directamente
./scripts/auto_update.sh
```

**El script automáticamente:**
- ✅ Crea un backup de tu versión actual
- ✅ Descarga la última versión desde GitHub
- ✅ Actualiza los archivos
- ✅ Recompila el proyecto
- ✅ Mantiene tu configuración personalizada

---

### 3️⃣ Verificación Automática al Inicio

COPENHEIMER verifica automáticamente si hay actualizaciones cada vez que lo ejecutas. Si detecta una nueva versión, muestra un mensaje informativo:

```
═══════════════════════════════════════════════════
   ✨ NUEVA ACTUALIZACIÓN DISPONIBLE ✨
═══════════════════════════════════════════════════
   Versión: v1.1.0 → v1.2.0

   Para actualizar, ejecuta:
   ./scripts/auto_update.sh
   o usa el comando:
   ./copenheimer_oxide --update
═══════════════════════════════════════════════════
```

---

## 🛠️ Métodos de Actualización Soportados

### Git Pull (Recomendado)

Si clonaste el repositorio con `git clone`, el sistema usa `git pull`:

```bash
git pull origin main
```

**Ventajas:**
- ⚡ Más rápido
- 🔄 Mantiene historial de git
- 📝 Preserva cambios locales (los guarda en stash)

### Descarga Directa

Si no usas git, el sistema descarga un tarball desde GitHub:

```bash
curl -L https://github.com/juancartek/1ueloke/archive/refs/heads/main.tar.gz
```

**Ventajas:**
- 📦 No requiere git instalado
- 🚀 Funciona en cualquier sistema
- 🔒 Garantiza versión limpia

---

## 💾 Sistema de Backup

Cada actualización crea un backup automático con timestamp:

```
.backup_20260924_175800/
├── version.txt
├── copenheimer_oxide/
│   └── src/
└── config/
    └── default.toml
```

### Restaurar desde Backup

Si algo sale mal, puedes restaurar manualmente:

```bash
# Encuentra el backup más reciente
ls -d .backup_* | tail -1

# Restaurar archivos
cp -r .backup_20260924_175800/* .

# Recompilar
cd copenheimer_oxide && cargo build --release
```

### Limpiar Backups Antiguos

Una vez verificado que la actualización funciona:

```bash
# Ver backups
ls -d .backup_*

# Eliminar un backup específico
rm -rf .backup_20260924_175800

# Eliminar todos los backups
rm -rf .backup_*
```

---

## 🔧 Configuración Avanzada

### Cambiar Repositorio

Edita las variables en `scripts/auto_update.sh`:

```bash
REPO_OWNER="juancartek"
REPO_NAME="1ueloke"
BRANCH="main"
```

### Cambiar Rama

Para actualizar desde una rama diferente:

```bash
BRANCH="development"  # o "beta", etc.
```

### Verificar Versión Actual

```bash
cat version.txt
```

---

## 📝 Archivo version.txt

El archivo `version.txt` controla la versión del proyecto:

```
1.1.0
```

**Formato:** `MAJOR.MINOR.PATCH` (Semantic Versioning)

- **MAJOR**: Cambios incompatibles con versiones anteriores
- **MINOR**: Nuevas funcionalidades compatibles
- **PATCH**: Corrección de bugs

---

## 🚨 Solución de Problemas

### Error: "No se pudo obtener la versión remota"

**Causa:** Sin conexión a Internet o GitHub inaccesible

**Solución:**
```bash
# Verificar conexión
ping github.com

# Intentar manualmente
curl -s https://api.github.com/repos/juancartek/1ueloke/releases/latest
```

### Error: "Script de actualización no encontrado"

**Causa:** Ejecutando desde directorio incorrecto

**Solución:**
```bash
# Verificar ubicación
pwd

# Ir al directorio raíz del proyecto
cd /path/to/1ueloke

# Verificar que existe el script
ls scripts/auto_update.sh
```

### Error durante la compilación

**Causa:** Dependencias faltantes o versión de Rust antigua

**Solución:**
```bash
# Actualizar Rust
rustup update

# Limpiar compilación anterior
cd copenheimer_oxide
cargo clean

# Recompilar
cargo build --release
```

### Restauración automática falló

**Causa:** Permiso insuficiente o backup corrupto

**Solución:**
```bash
# Verificar backups disponibles
ls -lah .backup_*

# Restaurar manualmente (ver sección de Backup arriba)

# Como último recurso: clonar repositorio limpio
cd ..
git clone https://github.com/juancartek/1ueloke.git 1ueloke_fresh
```

---

## 🔐 Seguridad

### Verificación de Integridad

El sistema descarga solo desde el repositorio oficial:
- `https://github.com/juancartek/1ueloke`
- `https://api.github.com/repos/juancartek/1ueloke`

### Backup Automático

Siempre se crea un backup antes de actualizar. Tu configuración y datos nunca se pierden.

### Sin Ejecución de Código Remoto

Los scripts solo descargan código fuente, que luego **tú** compilas localmente con Cargo.

---

## 📊 API de GitHub Utilizada

### Obtener Última Release

```bash
curl -s https://api.github.com/repos/juancartek/1ueloke/releases/latest
```

### Obtener version.txt desde Main

```bash
curl -s https://raw.githubusercontent.com/juancartek/1ueloke/main/version.txt
```

### Descargar Código Fuente

```bash
curl -L https://github.com/juancartek/1ueloke/archive/refs/heads/main.tar.gz
```

---

## ✅ Checklist de Actualización

Antes de actualizar:

- [ ] Backup de configuración personalizada
- [ ] Sin procesos de COPENHEIMER ejecutándose
- [ ] Conexión a Internet estable
- [ ] Espacio en disco suficiente (>100MB)

Después de actualizar:

- [ ] Verificar nueva versión: `cat version.txt`
- [ ] Probar compilación: `cargo build --release`
- [ ] Ejecutar tests: `cargo test`
- [ ] Eliminar backups antiguos si todo funciona

---

## 💡 Tips

1. **Actualiza regularmente** para obtener nuevas funcionalidades y correcciones
2. **Mantén backups** de tu configuración personalizada
3. **Lee el CHANGELOG** antes de actualizar para conocer cambios importantes
4. **Reporta problemas** en GitHub Issues si encuentras bugs

---

## 🔗 Links Útiles

- **Repositorio:** https://github.com/juancartek/1ueloke
- **Releases:** https://github.com/juancartek/1ueloke/releases
- **Issues:** https://github.com/juancartek/1ueloke/issues
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)

---

<div align="center">

**¿Preguntas? Abre un issue en GitHub**

[⬅️ Volver al README](README.md)

</div>
