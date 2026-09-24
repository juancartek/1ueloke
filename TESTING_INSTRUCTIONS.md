# 🧪 Instrucciones para Probar el Auto-Updater

## 📋 Preparación

Actualmente el proyecto está en **v5.8.0** con la nueva funcionalidad de base de datos.

---

## 🚀 Pasos para Probar

### 1. Sube esta versión a GitHub

```bash
cd /workspaces/1ueloke

# Ver todos los cambios
git status

# Añadir todo
git add .

# Commit
git commit -m "feat: v5.8.0 - Integración completa con bases de datos

- Soporte MongoDB Atlas y MongoDB local
- Guardado automático asíncrono de servidores encontrados
- Feature flag 'mongodb' para compilación opcional
- Documentación completa en DATABASE.md
- Ejemplos y tutoriales incluidos
- Sin impacto en performance del scanner
- Queries y estadísticas en tiempo real"

# Push a GitHub
git push origin main
```

### 2. Simula un usuario con versión antigua

Para probar el auto-updater, necesitas simular tener una versión anterior:

**Opción A: Usar otra máquina/VM**
- Clona el repo en otra máquina
- Haz checkout a un commit anterior (v5.7.0)
- Compila y ejecuta

**Opción B: Crear un branch de prueba**
```bash
# Crear branch de prueba
git checkout -b test-old-version

# Volver a commit anterior (v5.7.0)
git reset --hard HEAD~1  # O el commit específico de v5.7.0

# Compilar
cd copenheimer_oxide && cargo build --release

# Ejecutar
./target/release/copenheimer_oxide --check-updates
```

Deberías ver:
```
═══════════════════════════════════════════════════
   ✨ NUEVA ACTUALIZACIÓN DISPONIBLE ✨
═══════════════════════════════════════════════════
   Versión: v5.7.0 → v5.8.0
═══════════════════════════════════════════════════
```

**Opción C: Modificar version.txt temporalmente**
```bash
# Simular versión antigua
echo "5.7.0" > version.txt

# Verificar updates
./copenheimer_oxide/target/release/copenheimer_oxide --check-updates

# Debería detectar v5.8.0 disponible en GitHub
```

### 3. Probar actualización automática

```bash
# Opción A: Actualización directa
./copenheimer_oxide/target/release/copenheimer_oxide --update

# Opción B: Actualización desde el menú
./copenheimer_oxide/target/release/copenheimer_oxide
# Al iniciar, detectará automáticamente y preguntará si actualizar
```

**Verás:**
```
🔄 Descargando desde GitHub...
📂 Extrayendo archivos...
📝 Actualizando archivos...
🔨 Recompilando proyecto...
✅ Compilación exitosa
✨ Actualización completada - v5.8.0
```

### 4. Verificar que funcionó

```bash
# Verificar nueva versión
cat version.txt  # Debería mostrar 5.8.0

# Verificar que está actualizado
./copenheimer_oxide/target/release/copenheimer_oxide --check-updates
# Debería decir: "✅ Estás usando la última versión"

# Verificar que los nuevos archivos existen
ls -la copenheimer_oxide/src/database.rs  # Debería existir
ls -la DATABASE.md  # Debería existir
ls -la config/database.toml  # Debería existir
```

### 5. Probar nueva funcionalidad de BD (Opcional)

**Si quieres probar MongoDB Atlas:**

1. **Crear cuenta gratuita:**
   - Ve a https://www.mongodb.com/cloud/atlas
   - Regístrate (sin tarjeta)
   - Crea cluster M0 (gratis)

2. **Configurar:**
   ```bash
   # Editar config/database.toml
   nano config/database.toml
   
   # Cambiar:
   enabled = true
   connection_string = "tu-connection-string-de-atlas"
   ```

3. **Recompilar con MongoDB:**
   ```bash
   cd copenheimer_oxide
   cargo build --release --features mongodb
   ```

4. **Ejecutar:**
   ```bash
   ./target/release/copenheimer_oxide
   ```
   
   Deberías ver:
   ```
   📊 Conectando a base de datos...
   ✅ Conectado a MongoDB Atlas
   ```

---

## ✅ Checklist de Pruebas

- [ ] Push a GitHub completado
- [ ] Nueva versión visible en GitHub (v5.8.0)
- [ ] Simulación con versión antigua
- [ ] `--check-updates` detecta v5.8.0
- [ ] `--update` descarga correctamente
- [ ] Archivos se actualizan
- [ ] Recompilación automática funciona
- [ ] `version.txt` muestra 5.8.0 después
- [ ] Nuevos archivos presentes (database.rs, DATABASE.md)
- [ ] Programa funciona después de actualizar
- [ ] (Opcional) MongoDB conecta correctamente

---

## 📊 Resultados Esperados

### ✅ Éxito Total
- Auto-updater detecta nueva versión
- Descarga y actualiza archivos
- Recompila automáticamente
- Nueva funcionalidad (BD) disponible
- Sin errores ni warnings

### ⚠️ Si algo falla
- Revisa logs de error
- Verifica connection string de GitHub
- Asegúrate de que version.txt en GitHub está actualizado
- Comprueba que todos los archivos se subieron

---

## 💡 Tips

1. **GitHub releases:** Considera crear un release tag v5.8.0 en GitHub para que el auto-updater también detecte releases formales

2. **Prueba incremental:** Primero prueba `--check-updates`, luego `--update`

3. **Backup:** El auto-updater crea backups automáticos antes de actualizar

4. **Logs:** Revisa cualquier error durante la actualización

---

## 🎯 Objetivo de la Prueba

Verificar que:
1. ✅ El sistema detecta nuevas versiones en GitHub
2. ✅ Descarga correctamente
3. ✅ Actualiza sin perder configuración
4. ✅ Recompila automáticamente
5. ✅ Nueva funcionalidad funciona (BD)

---

## 📝 Reporte

Después de probar, verifica:
- ¿Detectó la actualización? ✅/❌
- ¿Descargó correctamente? ✅/❌
- ¿Actualizó archivos? ✅/❌
- ¿Recompiló exitosamente? ✅/❌
- ¿Nueva versión funciona? ✅/❌
- ¿BD se puede configurar? ✅/❌

---

<div align="center">

**¡Buena suerte con las pruebas! 🚀**

Si todo funciona, tendrás confirmación de que el sistema de
auto-actualización es completamente funcional.

</div>
