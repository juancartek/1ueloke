# 📊 Integración con Bases de Datos

COPENHEIMER ahora soporta almacenar los resultados de escaneo en bases de datos para análisis posterior, persistencia y consultas avanzadas.

---

## 🗄️ Proveedores Soportados

| Proveedor | Estado | Descripción |
|-----------|--------|-------------|
| **MongoDB Atlas** | ✅ Implementado | Base de datos NoSQL en la nube |
| **MongoDB Local** | ✅ Implementado | MongoDB instalado localmente |
| PostgreSQL | 🚧 En desarrollo | Base de datos relacional |
| MySQL | 🚧 En desarrollo | Base de datos relacional |
| SQLite | 🚧 En desarrollo | Base de datos embebida |

---

## 🚀 Inicio Rápido con MongoDB Atlas (Gratuito)

### Paso 1: Crear cuenta en MongoDB Atlas

1. Ve a https://www.mongodb.com/cloud/atlas
2. Crea una cuenta gratuita (no requiere tarjeta de crédito)
3. Crea un nuevo cluster (selecciona la opción FREE - M0)

### Paso 2: Configurar acceso

1. En el panel de Atlas, ve a **Database Access**
2. Crea un usuario de base de datos:
   - Username: `copenheimer`
   - Password: `TuContraseñaSegura123`
   - Roles: `Atlas admin` o `Read and write to any database`

3. Ve a **Network Access**
4. Añade tu IP o permite acceso desde cualquier IP:
   - Click en **Add IP Address**
   - Selecciona **Allow Access from Anywhere** (0.0.0.0/0)
   - Confirma

### Paso 3: Obtener Connection String

1. Ve a **Database** → **Connect**
2. Selecciona **Connect your application**
3. Copia el connection string (se ve así):
   ```
   mongodb+srv://copenheimer:<password>@cluster0.xxxxx.mongodb.net/?retryWrites=true&w=majority
   ```

### Paso 4: Configurar COPENHEIMER

Edita `config/database.toml`:

```toml
enabled = true
provider = "mongodb"
connection_string = "mongodb+srv://copenheimer:TuContraseñaSegura123@cluster0.xxxxx.mongodb.net/?retryWrites=true&w=majority"
database_name = "copenheimer"
collection_name = "minecraft_servers"
```

⚠️ **Importante:** Reemplaza `<password>` con tu contraseña real y `cluster0.xxxxx` con tu cluster real.

### Paso 5: Compilar con soporte MongoDB

```bash
cd copenheimer_oxide
cargo build --release --features mongodb
```

### Paso 6: Ejecutar

```bash
./target/release/copenheimer_oxide
```

**Salida esperada:**
```
📊 Conectando a base de datos...
✅ Conectado a MongoDB Atlas
📊 Base de datos: 0 servidores almacenados
```

---

## 🔧 Configuración Detallada

### Archivo `config/database.toml`

```toml
# Habilitar/deshabilitar almacenamiento en BD
enabled = true

# Proveedor de base de datos
provider = "mongodb"

# String de conexión
connection_string = "mongodb+srv://usuario:contraseña@cluster.mongodb.net/"

# Nombre de la base de datos
database_name = "copenheimer"

# Nombre de la colección
collection_name = "minecraft_servers"
```

### Estructura de Datos Almacenados

Cada servidor encontrado se guarda con la siguiente estructura:

```json
{
  "_id": "507f1f77bcf86cd799439011",
  "ip": "51.210.45.123",
  "domain": "mc.example.com",
  "version": "1.20.1",
  "online_players": 15,
  "max_players": 100,
  "motd": "Survival Server - Join Now!",
  "protocol": 761,
  "provider": "OVH/Gaming",
  "discovered_at": "2024-09-24T18:30:00.000Z"
}
```

---

## 💻 Ejemplos de Uso

### MongoDB Local

Si tienes MongoDB instalado localmente:

```toml
enabled = true
provider = "mongodb"
connection_string = "mongodb://localhost:27017"
database_name = "copenheimer"
collection_name = "servers"
```

### MongoDB con Autenticación

```toml
connection_string = "mongodb://username:password@localhost:27017/copenheimer?authSource=admin"
```

### Desactivar Base de Datos

Para volver a usar solo archivos:

```toml
enabled = false
```

---

## 📈 Consultas y Análisis

### Conectarse a MongoDB Atlas con CLI

```bash
mongosh "mongodb+srv://cluster0.xxxxx.mongodb.net/" --apiVersion 1 --username copenheimer
```

### Consultas Útiles

**Contar servidores totales:**
```javascript
use copenheimer
db.minecraft_servers.countDocuments()
```

**Servidores por versión:**
```javascript
db.minecraft_servers.aggregate([
  { $group: { _id: "$version", count: { $sum: 1 } } },
  { $sort: { count: -1 } }
])
```

**Servidores con más de 10 jugadores:**
```javascript
db.minecraft_servers.find({ online_players: { $gt: 10 } })
```

**Top 10 servidores más grandes:**
```javascript
db.minecraft_servers.find()
  .sort({ max_players: -1 })
  .limit(10)
```

**Servidores encontrados hoy:**
```javascript
db.minecraft_servers.find({
  discovered_at: {
    $gte: new Date(new Date().setHours(0,0,0,0))
  }
})
```

**Agrupar por proveedor:**
```javascript
db.minecraft_servers.aggregate([
  { $group: { _id: "$provider", count: { $sum: 1 } } },
  { $sort: { count: -1 } }
])
```

---

## 🔍 Monitoreo y Estadísticas

Durante el escaneo, COPENHEIMER muestra:

```
📊 Base de datos: 1523 servidores almacenados

[Servidor encontrado...]

📊 Total en BD: 1524 servidores
```

---

## ⚙️ Compilación

### Sin soporte de base de datos (más rápido):
```bash
cargo build --release
```

### Con MongoDB:
```bash
cargo build --release --features mongodb
```

### Con todos los features:
```bash
cargo build --release --features full
```

---

## 🚨 Solución de Problemas

### Error: "MongoDB no está habilitado"

**Solución:** Recompila con el feature:
```bash
cargo build --release --features mongodb
```

### Error de conexión a MongoDB Atlas

**Causas comunes:**
1. IP no está en la whitelist
   - Solución: Añade tu IP en Network Access
2. Usuario/contraseña incorrectos
   - Solución: Verifica credenciales en Database Access
3. Connection string incorrecto
   - Solución: Copia nuevamente desde Atlas

### Error: "connection timeout"

- Verifica tu conexión a Internet
- Verifica que el firewall permite conexiones salientes
- Verifica el connection string

### Colección vacía

- Verifica que `enabled = true`
- Verifica que la compilación incluye feature mongodb
- Revisa logs por errores durante el guardado

---

## 📊 Límites MongoDB Atlas (Tier Gratuito)

- **Storage:** 512 MB
- **RAM:** 512 MB  
- **Conexiones:** 500 simultáneas
- **Bandwidth:** Sin límite
- **Backups:** No automáticos

**Estimación:** ~1-2 millones de servidores almacenables

---

## 🔐 Seguridad

### Buenas Prácticas

1. **No subas credenciales a Git:**
   ```bash
   echo "config/database.toml" >> .gitignore
   ```

2. **Usa contraseñas fuertes:**
   - Mínimo 12 caracteres
   - Mezcla de mayúsculas, minúsculas, números y símbolos

3. **Restringe IPs:**
   - En producción, añade solo IPs específicas en Network Access

4. **Usa variables de entorno (opcional):**
   ```bash
   export COPENHEIMER_DB_URI="mongodb+srv://..."
   ```

---

## 📝 Roadmap

- [x] MongoDB Atlas/Local
- [ ] PostgreSQL
- [ ] MySQL
- [ ] SQLite
- [ ] Redis (caché)
- [ ] Elasticsearch (búsqueda)
- [ ] API REST para consultas
- [ ] Dashboard web

---

## 💡 Casos de Uso

### Investigación
Analiza distribución de versiones de Minecraft, tendencias de hosting, etc.

### Monitoreo
Rastrea servidores a lo largo del tiempo, detecta cambios.

### Análisis de Infraestructura
Identifica proveedores populares, rangos IP comunes.

### Machine Learning
Datos históricos para modelos de predicción.

---

## 🤝 Contribuciones

Para añadir soporte a otros proveedores:

1. Implementa trait `DatabaseProvider` en `src/database.rs`
2. Añade dependencia opcional en `Cargo.toml`
3. Añade feature flag
4. Actualiza documentación

---

<div align="center">

**¿Preguntas? Abre un issue en GitHub**

[⬅️ Volver al README](../README.md)

</div>
