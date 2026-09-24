#!/bin/bash
# update_checker.sh - Verifica si hay actualizaciones disponibles en GitHub
# Parte del sistema de auto-actualización de COPENHEIMER

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuración
REPO_OWNER="juancartek"
REPO_NAME="1ueloke"
GITHUB_API="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}"
VERSION_FILE="version.txt"

# Función para obtener la versión local
get_local_version() {
    if [ -f "$VERSION_FILE" ]; then
        cat "$VERSION_FILE"
    else
        echo "0.0.0"
    fi
}

# Función para obtener la última versión de GitHub
get_remote_version() {
    # Intenta obtener la última release de GitHub
    local latest_release=$(curl -s "${GITHUB_API}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' | sed 's/^v//')
    
    if [ -z "$latest_release" ] || [ "$latest_release" = "null" ]; then
        # Si no hay releases, intenta obtener del archivo version.txt en main
        latest_release=$(curl -s "https://raw.githubusercontent.com/${REPO_OWNER}/${REPO_NAME}/main/version.txt" 2>/dev/null)
    fi
    
    echo "$latest_release"
}

# Función para comparar versiones (formato: X.Y.Z)
version_gt() {
    test "$(printf '%s\n' "$@" | sort -V | head -n 1)" != "$1"
}

# Main
main() {
    echo -e "${CYAN}🔍 Verificando actualizaciones...${NC}"
    echo ""
    
    # Obtener versiones
    LOCAL_VERSION=$(get_local_version)
    REMOTE_VERSION=$(get_remote_version)
    
    if [ -z "$REMOTE_VERSION" ] || [ "$REMOTE_VERSION" = "null" ]; then
        echo -e "${RED}❌ No se pudo obtener la versión remota. Verifica tu conexión a Internet.${NC}"
        exit 1
    fi
    
    echo -e "${CYAN}📦 Versión local:${NC}  ${YELLOW}v${LOCAL_VERSION}${NC}"
    echo -e "${CYAN}☁️  Versión remota:${NC} ${YELLOW}v${REMOTE_VERSION}${NC}"
    echo ""
    
    # Comparar versiones
    if version_gt "$REMOTE_VERSION" "$LOCAL_VERSION"; then
        echo -e "${GREEN}✨ ¡Nueva actualización disponible!${NC}"
        echo -e "${GREEN}   v${LOCAL_VERSION} → v${REMOTE_VERSION}${NC}"
        echo ""
        echo -e "${CYAN}Para actualizar, ejecuta:${NC}"
        echo -e "  ${YELLOW}./scripts/auto_update.sh${NC}"
        echo ""
        exit 0
    elif [ "$REMOTE_VERSION" = "$LOCAL_VERSION" ]; then
        echo -e "${GREEN}✅ Estás usando la última versión${NC}"
        exit 0
    else
        echo -e "${YELLOW}⚠️  Tu versión es más reciente que la del repositorio${NC}"
        echo -e "${YELLOW}   (¿Estás en desarrollo?)${NC}"
        exit 0
    fi
}

main "$@"
