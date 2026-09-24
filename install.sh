#!/bin/bash
#
# Script de instalación automática de COPENHEIMER
# Para sistemas Linux, macOS y WSL
#

set -e  # Salir si hay errores

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # Sin color

# Función para imprimir con color
print_color() {
    color=$1
    message=$2
    echo -e "${color}${message}${NC}"
}

# Banner
echo ""
print_color "$RED" "   ____  _____  ____  ____  _      _      ____  _____  ____  ____  ____ "
print_color "$RED" "  /  _ \\/  __/ /  _ \\/  _ \\/ \\  /|\/ \\__/|/  _ \\/__ __\\/  _ \\/  _ \\/  _ \\"
print_color "$RED" "  | / \\||  \\   | / \\|| | //| |\\ ||| |\\/||| / \\|  / \\  | | //| / \\|| | //"
print_color "$RED" "  | \\_/||  /_  | \\_/|| |_\\\\| | \\||| |  ||| \\_/|  | |  | |_\\\\| \\_/|| |_\\\\"
print_color "$RED" "  \\____/\\____\\ \\____/\\____/\\_/  \\|\\_/  \\|\\____/  \\_/  \\____/\\____/\\____/"
echo ""
print_color "$BLUE" "  🔥 Instalador Automático v5.6.0"
echo ""

# Verificar sistema operativo
print_color "$YELLOW" "📋 Verificando sistema operativo..."
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    CYGWIN*)    MACHINE=Cygwin;;
    MINGW*)     MACHINE=MinGw;;
    *)          MACHINE="UNKNOWN:${OS}"
esac
print_color "$GREEN" "   ✓ Sistema: $MACHINE"

# Verificar si Rust está instalado
print_color "$YELLOW" "🦀 Verificando instalación de Rust..."
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_color "$GREEN" "   ✓ Rust ya está instalado: $RUST_VERSION"
else
    print_color "$YELLOW" "   ⚠️  Rust no está instalado. Instalando..."
    
    # Instalar Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Cargar entorno
    source $HOME/.cargo/env
    
    print_color "$GREEN" "   ✓ Rust instalado correctamente"
fi

# Verificar herramientas de compilación
print_color "$YELLOW" "🔧 Verificando herramientas de compilación..."
if command -v cc &> /dev/null || command -v gcc &> /dev/null; then
    print_color "$GREEN" "   ✓ Herramientas de compilación disponibles"
else
    print_color "$YELLOW" "   ⚠️  Instalando herramientas de compilación..."
    
    if [ "$MACHINE" = "Linux" ]; then
        # Detectar distribución
        if [ -f /etc/debian_version ]; then
            sudo apt-get update
            sudo apt-get install -y build-essential
        elif [ -f /etc/redhat-release ]; then
            sudo yum groupinstall -y "Development Tools"
        elif [ -f /etc/arch-release ]; then
            sudo pacman -S --noconfirm base-devel
        else
            print_color "$RED" "   ✗ No se pudo detectar la distribución. Instala 'build-essential' manualmente."
        fi
    elif [ "$MACHINE" = "Mac" ]; then
        xcode-select --install 2>/dev/null || true
    fi
    
    print_color "$GREEN" "   ✓ Herramientas de compilación instaladas"
fi

# Clonar o actualizar repositorio
print_color "$YELLOW" "📦 Descargando COPENHEIMER..."
if [ -d "1ueloke" ]; then
    print_color "$YELLOW" "   ℹ️  Repositorio ya existe. Actualizando..."
    cd 1ueloke
    git pull
else
    git clone https://github.com/juancartek/1ueloke.git
    cd 1ueloke
fi

cd copenheimer_oxide
print_color "$GREEN" "   ✓ Código descargado"

# Aumentar límite de archivos abiertos
print_color "$YELLOW" "⚙️  Configurando límites del sistema..."
ulimit -n 65000 2>/dev/null || print_color "$YELLOW" "   ⚠️  No se pudo aumentar el límite (ejecuta como root si es necesario)"

# Compilar en modo release
print_color "$YELLOW" "🔨 Compilando COPENHEIMER (esto puede tardar unos minutos)..."
cargo build --release

if [ $? -eq 0 ]; then
    print_color "$GREEN" "   ✓ Compilación exitosa"
else
    print_color "$RED" "   ✗ Error en la compilación"
    exit 1
fi

# Obtener tamaño del binario
BINARY_SIZE=$(du -h target/release/copenheimer_oxide | cut -f1)
print_color "$BLUE" "   📊 Tamaño del binario: $BINARY_SIZE"

# Crear enlace simbólico (opcional)
print_color "$YELLOW" "🔗 ¿Deseas instalar COPENHEIMER en el sistema? (s/n)"
read -r INSTALL_GLOBAL

if [[ "$INSTALL_GLOBAL" =~ ^[SsYy]$ ]]; then
    cargo install --path .
    print_color "$GREEN" "   ✓ COPENHEIMER instalado globalmente"
    print_color "$BLUE" "   Ahora puedes ejecutarlo con: copenheimer_oxide"
fi

# Ejecutar tests
print_color "$YELLOW" "🧪 ¿Deseas ejecutar los tests? (s/n)"
read -r RUN_TESTS

if [[ "$RUN_TESTS" =~ ^[SsYy]$ ]]; then
    cargo test
    print_color "$GREEN" "   ✓ Tests completados"
fi

# Resumen final
echo ""
print_color "$GREEN" "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_color "$GREEN" "✅ ¡INSTALACIÓN COMPLETADA EXITOSAMENTE!"
print_color "$GREEN" "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
print_color "$BLUE" "📍 Ubicación del binario:"
echo "   $(pwd)/target/release/copenheimer_oxide"
echo ""
print_color "$BLUE" "▶️  Para ejecutar COPENHEIMER:"
if [[ "$INSTALL_GLOBAL" =~ ^[SsYy]$ ]]; then
    print_color "$YELLOW" "   copenheimer_oxide"
else
    print_color "$YELLOW" "   ./target/release/copenheimer_oxide"
    print_color "$YELLOW" "   # O con Cargo:"
    print_color "$YELLOW" "   cargo run --release"
fi
echo ""
print_color "$BLUE" "📚 Documentación:"
print_color "$YELLOW" "   • README.md  - Guía general"
print_color "$YELLOW" "   • INSTALL.md - Guía de instalación detallada"
print_color "$YELLOW" "   • CONTRIBUTING.md - Guía de contribución"
echo ""
print_color "$BLUE" "🎯 Ejemplos de uso:"
print_color "$YELLOW" "   cargo run --release"
print_color "$YELLOW" "   cargo run --release -- --check 51.210.45.123"
print_color "$YELLOW" "   cargo run --example basic_scan"
echo ""
print_color "$GREEN" "🔥 ¡Disfruta escaneando servidores de Minecraft! 🦀"
echo ""
