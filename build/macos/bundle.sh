#!/bin/bash
set -e

# ================= 配置项 =================
APP_NAME="ffmpeg_egui_hlw"
TARGET_DIR="target/release"
EXECUTABLE="$TARGET_DIR/$APP_NAME"

# 输出目录配置
DIST_DIR="dist"
BUNDLE_DIR="$DIST_DIR/$APP_NAME.app"
CONTENTS="$BUNDLE_DIR/Contents"
MACOS="$CONTENTS/MacOS"
FRAMEWORKS="$CONTENTS/Frameworks"
RESOURCES="$CONTENTS/Resources"
DMG_NAME="$DIST_DIR/$APP_NAME.dmg"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warn() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# ================= 辅助函数：获取绝对路径 =================
get_abs_path() {
    local target="$1"
    if [ -f "$target" ]; then
        local dir=$(dirname "$target")
        local file=$(basename "$target")
        echo "$(cd "$dir" >/dev/null 2>&1 && pwd)/$file"
    else
        echo "$target"
    fi
}

# ================= 1. 环境与构建检查 =================
for tool in otool install_name_tool hdiutil cargo; do
    if ! command -v "$tool" &>/dev/null; then
        log_error "缺少必要工具: $tool"
        exit 1
    fi
done

if [ ! -f "$EXECUTABLE" ]; then
    log_warn "可执行文件不存在，正在尝试构建 Release 版本..."
    cargo build --release
fi

# ================= 2. 清理与目录创建 =================
log_info "清理旧构建..."
rm -rf "$DIST_DIR"
mkdir -p "$MACOS" "$FRAMEWORKS" "$RESOURCES"

# ================= 3. 复制核心文件 =================
log_info "复制可执行文件..."
cp "$EXECUTABLE" "$MACOS/$APP_NAME"
chmod +x "$MACOS/$APP_NAME"

ICON_PATH="assets/AppIcon.icns"
if [ -f "$ICON_PATH" ]; then
    log_info "复制应用图标..."
    cp "$ICON_PATH" "$RESOURCES/"
fi

# ================= 4. 智能依赖解析与复制 =================
log_info "开始解析并打包动态库..."

is_system_lib() {
    local lib_path="$1"
    [[ "$lib_path" == /System/* ]] || [[ "$lib_path" == /usr/lib/* ]]
}

PROCESSED_LIBS_STR=":"

bundle_libs() {
    local binary="$1"
    
    # [核心修复]: 使用 sed 's/ (.*//' 替代 awk -F' ('
    # otool -L 输出格式:
    # /path/to/lib.dylib (compatibility version 1.0.0, ...)
    local dependencies=$(otool -L "$binary" | sed '1d' | grep -v "$binary" | sed 's/ (.*//' | xargs)
    
    for dep_path in $dependencies; do
        if [[ "$dep_path" == @* ]]; then continue; fi
        if is_system_lib "$dep_path"; then continue; fi

        local real_path=$(get_abs_path "$dep_path")
        
        if [ ! -f "$real_path" ]; then
            continue
        fi
        
        local lib_name=$(basename "$real_path")
        local dest_path="$FRAMEWORKS/$lib_name"

        if [[ "$PROCESSED_LIBS_STR" != *":$real_path:"* ]]; then
            log_info "  → 打包依赖: $lib_name"
            
            cp -f "$real_path" "$dest_path"
            chmod 755 "$dest_path"
            
            PROCESSED_LIBS_STR="${PROCESSED_LIBS_STR}${real_path}:"
            
            bundle_libs "$dest_path"
        fi
    done
}

bundle_libs "$MACOS/$APP_NAME"

# ================= 5. 修正库引用路径 =================
log_info "修正动态库引用路径..."

install_name_tool -add_rpath "@executable_path/../Frameworks" "$MACOS/$APP_NAME" 2>/dev/null || true

# 修正 Frameworks 里的库
find "$FRAMEWORKS" -name "*.dylib" | while read -r lib_file; do
    lib_name=$(basename "$lib_file")
    install_name_tool -id "@rpath/$lib_name" "$lib_file"

    # [核心修复]: 同样使用 sed 替代 awk
    otool -L "$lib_file" | sed '1d' | grep -v "$lib_name" | sed 's/ (.*//' | xargs | while read -r dep; do
        if [[ "$dep" != @* ]] && [[ "$dep" != /System/* ]] && [[ "$dep" != /usr/lib/* ]]; then
            dep_name=$(basename "$dep")
            install_name_tool -change "$dep" "@rpath/$dep_name" "$lib_file" 2>/dev/null
        fi
    done
done

# 修正主程序的引用
# [核心修复]: 同样使用 sed 替代 awk
otool -L "$MACOS/$APP_NAME" | sed '1d' | grep -v "/System/" | grep -v "/usr/lib/" | sed 's/ (.*//' | xargs | while read -r dep; do
    if [[ "$dep" != @* ]]; then
        dep_name=$(basename "$dep")
        install_name_tool -change "$dep" "@rpath/$dep_name" "$MACOS/$APP_NAME"
    fi
done

# ================= 6. 生成 Info.plist =================
log_info "生成 Info.plist..."
cat > "$CONTENTS/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.ffmpeg-egui.$APP_NAME</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# ================= 7. 签名与打包 DMG =================
log_info "执行签名 (Ad-hoc)..."
codesign --force --deep --sign - "$BUNDLE_DIR"

log_info "生成 DMG..."
DMG_ROOT="$DIST_DIR/dmg_root"
mkdir -p "$DMG_ROOT"
cp -R "$BUNDLE_DIR" "$DMG_ROOT/"
ln -s /Applications "$DMG_ROOT/Applications"
hdiutil create -volname "$APP_NAME" -srcfolder "$DMG_ROOT" -ov -format UDZO "$DMG_NAME" >/dev/null
rm -rf "$DMG_ROOT"

log_success "打包完成: $DMG_NAME"