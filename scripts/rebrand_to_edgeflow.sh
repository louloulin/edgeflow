#!/bin/bash
# EdgeFlow品牌重构自动化脚本
# 将Proksi项目重构为EdgeFlow品牌

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查是否在项目根目录
check_project_root() {
    if [ ! -f "Cargo.toml" ] || [ ! -d "crates" ]; then
        log_error "请在项目根目录运行此脚本"
        exit 1
    fi
}

# 更新Cargo.toml文件
update_cargo_files() {
    log_info "更新Cargo.toml文件..."
    
    # 查找所有Cargo.toml文件
    find . -name "Cargo.toml" -type f | while read -r file; do
        log_info "处理文件: $file"
        
        # 使用sed进行替换
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS版本
            sed -i '' -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/PROKSI/EDGEFLOW/g' \
                -e 's/"A high-performance AI gateway"/"EdgeFlow - 边缘AI数据流处理平台"/' \
                -e 's/luizfonseca\/proksi/edgeflow\/edgeflow/' \
                -e 's/authors = \[".*"\]/authors = ["EdgeFlow Team <team@edgeflow.ai>"]/' \
                -e 's/homepage = ".*"/homepage = "https:\/\/edgeflow.ai"/' \
                -e 's/repository = ".*"/repository = "https:\/\/github.com\/edgeflow\/edgeflow"/' \
                "$file"
        else
            # Linux版本
            sed -i -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/PROKSI/EDGEFLOW/g' \
                -e 's/"A high-performance AI gateway"/"EdgeFlow - 边缘AI数据流处理平台"/' \
                -e 's/luizfonseca\/proksi/edgeflow\/edgeflow/' \
                -e 's/authors = \[".*"\]/authors = ["EdgeFlow Team <team@edgeflow.ai>"]/' \
                -e 's/homepage = ".*"/homepage = "https:\/\/edgeflow.ai"/' \
                -e 's/repository = ".*"/repository = "https:\/\/github.com\/edgeflow\/edgeflow"/' \
                "$file"
        fi
    done
    
    log_success "Cargo.toml文件更新完成"
}

# 更新源代码文件
update_source_code() {
    log_info "更新源代码文件..."
    
    # 查找所有Rust源文件
    find crates src -name "*.rs" -type f 2>/dev/null | while read -r file; do
        log_info "处理源文件: $file"
        
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS版本
            sed -i '' -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/PROKSI/EDGEFLOW/g' \
                -e 's/proksi_/edgeflow_/g' \
                -e 's/PROKSI_/EDGEFLOW_/g' \
                -e 's/"proksi"/"edgeflow"/g' \
                "$file"
        else
            # Linux版本
            sed -i -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/PROKSI/EDGEFLOW/g' \
                -e 's/proksi_/edgeflow_/g' \
                -e 's/PROKSI_/EDGEFLOW_/g' \
                -e 's/"proksi"/"edgeflow"/g' \
                "$file"
        fi
    done
    
    log_success "源代码文件更新完成"
}

# 更新配置文件
update_config_files() {
    log_info "更新配置文件..."
    
    # 更新HCL配置文件
    find examples -name "*.hcl" -type f 2>/dev/null | while read -r file; do
        log_info "处理HCL配置: $file"
        
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/service_name = "proksi"/service_name = "edgeflow"/' \
                -e 's/# Proksi/# EdgeFlow/g' \
                "$file"
        else
            sed -i -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/service_name = "proksi"/service_name = "edgeflow"/' \
                -e 's/# Proksi/# EdgeFlow/g' \
                "$file"
        fi
    done
    
    # 更新YAML配置文件
    find examples -name "*.yaml" -type f 2>/dev/null | while read -r file; do
        log_info "处理YAML配置: $file"
        
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/service_name: proksi/service_name: edgeflow/' \
                "$file"
        else
            sed -i -e 's/proksi/edgeflow/g' \
                -e 's/Proksi/EdgeFlow/g' \
                -e 's/service_name: proksi/service_name: edgeflow/' \
                "$file"
        fi
    done
    
    log_success "配置文件更新完成"
}

# 更新文档文件
update_documentation() {
    log_info "更新文档文件..."
    
    # 更新Markdown文件
    find . -name "*.md" -type f | while read -r file; do
        log_info "处理文档: $file"
        
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' -e 's/Proksi/EdgeFlow/g' \
                -e 's/proksi/edgeflow/g' \
                -e 's/PROKSI/EDGEFLOW/g' \
                -e 's/github\.com\/luizfonseca\/proksi/github.com\/edgeflow\/edgeflow/g' \
                "$file"
        else
            sed -i -e 's/Proksi/EdgeFlow/g' \
                -e 's/proksi/edgeflow/g' \
                -e 's/PROKSI/EDGEFLOW/g' \
                -e 's/github\.com\/luizfonseca\/proksi/github.com\/edgeflow\/edgeflow/g' \
                "$file"
        fi
    done
    
    log_success "文档文件更新完成"
}

# 重命名目录和文件
rename_directories() {
    log_info "重命名目录和文件..."
    
    # 重命名主要目录
    if [ -d "crates/proksi" ]; then
        mv "crates/proksi" "crates/edgeflow-gateway"
        log_success "重命名 crates/proksi -> crates/edgeflow-gateway"
    fi
    
    if [ -d "crates/plugin_request_id" ]; then
        mv "crates/plugin_request_id" "crates/edgeflow-request-id"
        log_success "重命名 crates/plugin_request_id -> crates/edgeflow-request-id"
    fi
    
    if [ -d "crates/plugins_api" ]; then
        mv "crates/plugins_api" "crates/edgeflow-plugins-api"
        log_success "重命名 crates/plugins_api -> crates/edgeflow-plugins-api"
    fi
    
    log_success "目录和文件重命名完成"
}

# 更新工作空间配置
update_workspace() {
    log_info "更新工作空间配置..."
    
    if [ -f "Cargo.toml" ]; then
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' -e 's/"crates\/proksi"/"crates\/edgeflow-gateway"/' \
                -e 's/"crates\/plugin_request_id"/"crates\/edgeflow-request-id"/' \
                -e 's/"crates\/plugins_api"/"crates\/edgeflow-plugins-api"/' \
                "Cargo.toml"
        else
            sed -i -e 's/"crates\/proksi"/"crates\/edgeflow-gateway"/' \
                -e 's/"crates\/plugin_request_id"/"crates\/edgeflow-request-id"/' \
                -e 's/"crates\/plugins_api"/"crates\/edgeflow-plugins-api"/' \
                "Cargo.toml"
        fi
        log_success "工作空间配置更新完成"
    fi
}

# 主函数
main() {
    log_info "🚀 开始EdgeFlow品牌重构..."
    
    check_project_root
    
    update_cargo_files
    update_source_code
    update_config_files
    update_documentation
    rename_directories
    update_workspace
    
    log_success "✅ EdgeFlow品牌重构完成!"
    log_info "请运行 'cargo check' 验证配置正确性"
}

# 运行主函数
main "$@"
