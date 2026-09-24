# 🚀 Release v5.8.0 - Integración con Bases de Datos

## 🎯 Nueva Funcionalidad Principal

### 📊 Almacenamiento en Bases de Datos

COPENHEIMER ahora puede guardar automáticamente todos los servidores encontrados en bases de datos para análisis posterior.

**Proveedores soportados:**
- ✅ **MongoDB Atlas** (nube, gratuito)
- ✅ **MongoDB Local**
- 🚧 PostgreSQL (próximamente)
- 🚧 MySQL (próximamente)  
- 🚧 SQLite (próximamente)

---

## 📦 Qué incluye esta actualización

### Archivos nuevos:
- `src/database.rs` - Módulo completo de BD (284 líneas)
- `config/database.toml` - Configuración de conexión
- `DATABASE.md` - Guía completa paso a paso (360 líneas)
- `examples/database_demo.rs` - Ejemplo funcional

### Mejoras técnicas:
- Feature flag `mongodb` para compilación opcional
- Guardado asíncrono sin afectar performance
- Verificación de duplicados
- Estadísticas en tiempo real
- Timestamps automáticos con `chrono`

---

## 🚀 Cómo usar

### Paso 1: Compilar con soporte MongoDB

```bash
cd copenheimer_oxide
cargo build --release --features mongodb
```

### Paso 2: Configurar conexión

Edita `config/database.toml`:

```toml
enabled = true
provider = "mongodb"
connection_string = "mongodb+srv://usuario:contraseña@cluster.mongodb.net/"
database_name = "copenheimer"
collection_name = "minecraft_servers"
```

### Paso 3: Ejecutar

```bash
./target/release/copenheimer_oxide
```

**Verás:**
```
📊 Conectando a base de datos...
✅ Conectado a MongoDB Atlas
📊 Base de datos: 0 servidores almacenados
```

---

## 📊 Estructura de datos

Cada servidor se guarda con:

```json
{
  "ip": "51.210.45.123",
  "domain": "mc.example.com",
  "version": "1.20.1",
  "online_players": 15,
  "max_players": 100,
  "motd": "Survival Server",
  "protocol": 761,
  "provider": "OVH/Gaming",
  "discovered_at": "2024-09-24T18:30:00.000Z"
}
```

---

## 🎓 Tutorial MongoDB Atlas (Gratuito)

### 1. Crear cuenta
- Ve a https://www.mongodb.com/cloud/atlas
- Registra una cuenta gratuita (sin tarjeta)

### 2. Crear cluster
- Crea un cluster M0 (GRATIS)
- Selecciona región cercana

### 3. Configurar acceso
- **Database Access**: Crea usuario con contraseña
- **Network Access**: Permite tu IP o 0.0.0.0/0

### 4. Obtener connection string
- Click en "Connect" → "Connect your application"
- Copia el URI

### 5. Configurar y ejecutar
Ver sección "Cómo usar" arriba

---

## 📈 Consultas Útiles

Conecta con mongosh:
```bash
mongosh "mongodb+srv://cluster.mongodb.net/" --username copenheimer
```

**Total de servidores:**
```javascript
db.minecraft_servers.countDocuments()
```

**Servidores por versión:**
```javascript
db.minecraft_servers.aggregate([
  { $group: { _id: "$version", count: { $sum: 1 } } },
  { $sort: { count: -1 } }
])
```

**Top 10 más grandes:**
```javascript
db.minecraft_servers.find().sort({ max_players: -1 }).limit(10)
```

---

## 🔧 Compilación

**Sin base de datos (normal):**
```bash
cargo build --release
```

**Con MongoDB:**
```bash
cargo build --release --features mongodb
```

**Todas las features:**
```bash
cargo build --release --features full
```

---

## ⚡ Performance

El guardado en BD es **completamente asíncrono** y no afecta la velocidad del scanner:

- ✅ Sin impacto en PPS (packets per second)
- ✅ Guardado en background
- ✅ Manejo de errores sin crashear
- ✅ Continúa escaneando aunque BD falle

---

## 📝 Documentación

Lee la guía completa en `DATABASE.md` para:
- Tutorial paso a paso con screenshots
- Consultas avanzadas
- Solución de problemas
- Casos de uso
- Seguridad y buenas prácticas

---

## 🧪 Probar el sistema de auto-actualización

Esta actualización es perfecta para probar la nueva función de auto-actualización:

### Prueba manual:

1. **Antes de actualizar:**
   ```bash
   cat version.txt  # Debería mostrar 5.7.0 o anterior
   ```

2. **Verificar actualización:**
   ```bash
   ./copenheimer_oxide --check-updates
   ```
   
   Debería mostrar:
   ```
   ✨ NUEVA ACTUALIZACIÓN DISPONIBLE ✨
   Versión: v5.7.0 → v5.8.0
   ```

3. **Actualizar:**
   ```bash
   ./copenheimer_oxide --update
   ```

4. **Verificar nueva versión:**
   ```bash
   cat version.txt  # Debería mostrar 5.8.0
   ./copenheimer_oxide --check-updates  # Debería decir "última versión"
   ```

---

## 🎁 Bonus: Sin scripts externos

Todo está integrado en el binario:
- ✅ Descarga automática desde GitHub
- ✅ Instalación automática
- ✅ Recompilación automática
- ✅ Sin necesidad de scripts bash

---

## 📌 Changelog Completo

Ver `CHANGELOG.md` para detalles completos de todos los cambios en v5.8.0, v5.7.0 y versiones anteriores.

---

## 🐛 Reportar Problemas

Si encuentras algún problema:
1. Abre un issue en GitHub
2. Incluye la salida de `./copenheimer_oxide --check-updates`
3. Describe los pasos para reproducir

---

<div align="center">

**🔥 COPENHEIMER v5.8.0 🔥**

[GitHub](https://github.com/juancartek/1ueloke) • [Issues](https://github.com/juancartek/1ueloke/issues) • [Documentación](https://github.com/juancartek/1ueloke#readme)

</div>
