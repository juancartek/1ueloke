#!/bin/bash
# auto_update.sh - Descarga e instala actualizaciones desde GitHub
# Parte del sistema de auto-actualización de COPENHEIMER

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración
REPO_OWNER="juancartek"
REPO_NAME="1ueloke"
GITHUB_REPO="https://github.com/${REPO_OWNER}/${REPO_NAME}"
BRANCH="main"
VERSION_FILE="version.txt"
BACKUP_DIR=".backup_$(date +%Y%m%d_%H%M%S)"

# Función para crear backup
create_backup() {
    echo -e "${CYAN}📦 Creando backup...${NC}"
    mkdir -p "$BACKUP_DIR"
    
    # Backup de archivos importantes
    [ -f "$VERSION_FILE" ] && cp "$VERSION_FILE" "$BACKUP_DIR/"
    [ -d "copenheimer_oxide" ] && cp -r "copenheimer_oxide" "$BACKUP_DIR/" 2>/dev/null || true
    [ -f "config/default.toml" ] && cp "config/default.toml" "$BACKUP_DIR/" 2>/dev/null || true
    
    echo -e "${GREEN}✅ Backup creado en: ${BACKUP_DIR}${NC}"
}

# Función para restaurar backup
restore_backup() {
    if [ -d "$BACKUP_DIR" ]; then
        echo -e "${YELLOW}⚠️  Restaurando desde backup...${NC}"
        cp -r "$BACKUP_DIR"/* . 2>/dev/null || true
        echo -e "${GREEN}✅ Backup restaurado${NC}"
    fi
}

# Función para actualizar usando git pull
update_with_git() {
    echo -e "${CYAN}🔄 Actualizando con git pull...${NC}"
    
    # Verificar si es un repositorio git
    if [ ! -d ".git" ]; then
        echo -e "${RED}❌ No es un repositorio git. Usa el método de descarga.${NC}"
        return 1
    fi
    
    # Guardar cambios locales si existen
    if ! git diff-index --quiet HEAD --; then
        echo -e "${YELLOW}⚠️  Hay cambios locales. Guardando en stash...${NC}"
        git stash save "Auto-update backup $(date)"
    fi
    
    # Actualizar
    git fetch origin
    git pull origin "$BRANCH"
    
    echo -e "${GREEN}✅ Actualización completada con git${NC}"
}

# Función para actualizar descargando desde GitHub
update_with_download() {
    echo -e "${CYAN}📥 Descargando última versión desde GitHub...${NC}"
    
    TEMP_DIR=$(mktemp -d)
    ARCHIVE="$TEMP_DIR/repo.tar.gz"
    
    # Descargar tarball
    echo -e "${CYAN}⬇️  Descargando...${NC}"
    if ! curl -L -o "$ARCHIVE" "${GITHUB_REPO}/archive/refs/heads/${BRANCH}.tar.gz"; then
        echo -e "${RED}❌ Error descargando desde GitHub${NC}"
        rm -rf "$TEMP_DIR"
        return 1
    fi
    
    # Extraer
    echo -e "${CYAN}📂 Extrayendo archivos...${NC}"
    tar -xzf "$ARCHIVE" -C "$TEMP_DIR"
    
    # Copiar archivos (excluyendo config personalizado)
    EXTRACT_DIR="$TEMP_DIR/${REPO_NAME}-${BRANCH}"
    
    # Actualizar código fuente
    echo -e "${CYAN}📝 Actualizando archivos...${NC}"
    cp -r "$EXTRACT_DIR/copenheimer_oxide"/* copenheimer_oxide/ 2>/dev/null || true
    cp "$EXTRACT_DIR/version.txt" . 2>/dev/null || true
    cp "$EXTRACT_DIR"/*.md . 2>/dev/null || true
    cp "$EXTRACT_DIR"/*.sh . 2>/dev/null || true
    
    # Copiar scripts si existen
    [ -d "$EXTRACT_DIR/scripts" ] && cp -r "$EXTRACT_DIR/scripts" . 2>/dev/null || true
    
    # Limpiar
    rm -rf "$TEMP_DIR"
    
    echo -e "${GREEN}✅ Archivos actualizados${NC}"
}

# Función para recompilar el proyecto
rebuild_project() {
    echo -e "${CYAN}🔨 Recompilando proyecto...${NC}"
    
    cd copenheimer_oxide
    
    if command -v cargo &> /dev/null; then
        cargo build --release
        echo -e "${GREEN}✅ Proyecto recompilado exitosamente${NC}"
        echo -e "${CYAN}📍 Binario en: ${YELLOW}target/release/copenheimer_oxide${NC}"
    else
        echo -e "${YELLOW}⚠️  Cargo no encontrado. Recompila manualmente con:${NC}"
        echo -e "   ${YELLOW}cd copenheimer_oxide && cargo build --release${NC}"
    fi
    
    cd ..
}

# Función principal
main() {
    echo -e "${BLUE}"
    echo "╔════════════════════════════════════════════════════╗"
    echo "║     🔥 COPENHEIMER AUTO-UPDATER 🔥                ║"
    echo "╚════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    # Verificar conexión a Internet
    if ! curl -s --head --request GET "https://github.com" | grep "200 OK" > /dev/null; then
        echo -e "${RED}❌ No hay conexión a Internet${NC}"
        exit 1
    fi
    
    # Crear backup
    create_backup
    
    # Intentar actualizar
    echo ""
    if [ -d ".git" ]; then
        echo -e "${CYAN}🔍 Detectado repositorio git${NC}"
        if update_with_git; then
            UPDATE_SUCCESS=true
        else
            echo -e "${YELLOW}⚠️  Intentando método alternativo...${NC}"
            UPDATE_SUCCESS=false
        fi
    else
        UPDATE_SUCCESS=false
    fi
    
    # Si git falló o no está disponible, usar descarga directa
    if [ "$UPDATE_SUCCESS" = false ]; then
        if ! update_with_download; then
            echo -e "${RED}❌ Error durante la actualización${NC}"
            restore_backup
            exit 1
        fi
    fi
    
    # Recompilar
    echo ""
    rebuild_project
    
    # Mostrar nueva versión
    echo ""
    if [ -f "$VERSION_FILE" ]; then
        NEW_VERSION=$(cat "$VERSION_FILE")
        echo -e "${GREEN}✨ Actualización completada exitosamente${NC}"
        echo -e "${GREEN}📦 Nueva versión: ${YELLOW}v${NEW_VERSION}${NC}"
    fi
    
    echo ""
    echo -e "${CYAN}💡 Tip: El backup se guardó en ${YELLOW}${BACKUP_DIR}${NC}"
    echo -e "${CYAN}   Puedes eliminarlo si todo funciona correctamente${NC}"
    echo ""
    echo -e "${GREEN}🚀 ¡Listo para usar COPENHEIMER actualizado!${NC}"
}

# Manejo de errores
trap 'echo -e "${RED}❌ Error durante la actualización${NC}"; restore_backup; exit 1' ERR

main "$@"
