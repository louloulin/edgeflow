
![GitHub Release](https://img.shields.io/github/v/release/edgeflow/edgeflow?style=for-the-badge)
![Crates.io MSRV](https://img.shields.io/crates/msrv/edgeflow?style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/edgeflow?style=for-the-badge)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/edgeflow?style=for-the-badge)](https://crates.io/crates/edgeflow)

# EdgeFlow

🚀 **边缘AI数据流处理平台** - 下一代边缘AI基础设施

<div align="center">
  <img src="./assets/edgeflow-logo.png" alt="EdgeFlow Logo" width="200"/>
</div>

## 🎯 关于EdgeFlow

EdgeFlow是一个高性能的边缘AI网关和推理平台，通过深度集成WebAssembly和LlamaEdge技术，为企业提供完整的边缘AI解决方案。基于Rust和Pingora构建，专为现代AI应用和边缘计算场景设计。

**核心理念**: "让AI在边缘流动起来" - AI at the Edge, Flow at Scale


## ✨ 核心特性

### 🤖 AI原生设计
- **边缘推理**: 本地AI模型推理，延迟<10ms
- **智能路由**: 基于AI能力的智能请求路由
- **模型管理**: 自动模型发现、缓存和负载均衡
- **多模态支持**: 文本、图像、音频、视频统一处理
- **LLM聚合**: 多模型响应聚合和最优选择

### ⚡ 高性能网关
- **极致性能**: 基于Rust + Pingora，支持10万QPS
- **WebAssembly**: 安全的WASM插件生态系统
- **自动HTTPS**: Let's Encrypt自动证书管理
- **负载均衡**: 智能负载均衡和故障转移
- **智能缓存**: 语义哈希优化缓存命中率

### 🌐 边缘优先
- **边缘部署**: 专为边缘计算场景优化
- **离线能力**: 支持完全离线的AI推理
- **资源高效**: 相比Docker减少50%资源占用
- **即时启动**: 毫秒级冷启动时间
- **分布式**: 支持分布式部署和集群

### 🔌 可扩展插件系统
- **16个内置插件**: 认证、缓存、监控、AI处理等
- **WASM插件**: 支持多语言插件开发
- **热加载**: 无需重启即可更新插件
- **插件市场**: 社区驱动的插件生态

### 🛡️ 企业级安全
- **AI安全**: 提示注入检测和内容过滤
- **OAuth2集成**: 支持GitHub、WorkOS等认证提供商
- **零信任架构**: 端到端加密和访问控制
- **合规支持**: 满足GDPR、SOC2等合规要求
- **多租户隔离**: 安全的租户分离和配额管理

### 📊 全面可观测性
- **实时监控**: 性能指标和健康检查
- **AI分析**: AI请求响应的深度分析
- **Web界面**: 基于Web的管理和分析界面
- **Prometheus集成**: 与监控系统无缝集成
- **审计日志**: 完整的合规和安全分析日志

## 🚀 快速开始

### 安装EdgeFlow

#### 方式1: 使用Cargo安装
```bash
cargo install edgeflow
```

#### 方式2: 从源码构建
```bash
git clone https://github.com/edgeflow/edgeflow.git
cd edgeflow
cargo build --release
```

#### 方式3: Docker部署
```bash
docker run -p 80:80 -p 443:443 edgeflow/edgeflow
```

### 基础配置

创建配置文件 `edgeflow.hcl`:
```hcl
# EdgeFlow配置文件
service_name = "edgeflow"

# 服务器配置
server {
  https_address = "0.0.0.0:443"
  http_address = "0.0.0.0:80"
}

# 边缘AI推理配置
inference {
  enabled = true
  models_path = "./models"
  api_port = 8080
}

# 插件配置
plugins = [
  {
    name = "edgeflow_request_id"
    enabled = true
  },
  {
    name = "edgeflow_ai_gateway"
    enabled = true
    config = {
      inference_timeout = 30000
      model_cache_size = 5
    }
  }
]
```

### 启动服务
```bash
# 使用配置文件启动
edgeflow start --config edgeflow.hcl

# 或使用默认配置
edgeflow start
```

### 验证安装
```bash
# 检查服务状态
curl http://localhost/health

# 查看AI模型列表
curl http://localhost:8080/v1/models

# 测试AI推理
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama2-7b",
    "messages": [{"role": "user", "content": "Hello, EdgeFlow!"}]
  }'
```

## 📚 文档和资源

### 官方文档
- [快速开始指南](https://docs.edgeflow.ai/quick-start)
- [配置参考](https://docs.edgeflow.ai/configuration)
- [插件开发](https://docs.edgeflow.ai/plugins)
- [API文档](https://docs.edgeflow.ai/api)
- [部署指南](https://docs.edgeflow.ai/deployment)

### 社区资源
- [GitHub仓库](https://github.com/edgeflow/edgeflow)
- [技术博客](https://blog.edgeflow.ai)
- [社区论坛](https://community.edgeflow.ai)
- [Discord频道](https://discord.gg/edgeflow)

## 🏢 企业版

EdgeFlow提供企业级功能和支持：

### 企业级特性
- **24/7技术支持**: 专业技术团队支持
- **高可用部署**: 多区域容灾部署
- **企业级安全**: 高级安全功能和合规支持
- **定制开发**: 根据需求定制功能
- **专业培训**: 技术培训和认证服务

### 联系我们
- **官网**: [edgeflow.ai/enterprise](https://edgeflow.ai/enterprise)
- **邮箱**: enterprise@edgeflow.ai
- **电话**: +1-800-EDGEFLOW

## 🤝 贡献指南

我们欢迎社区贡献！请查看我们的[贡献指南](CONTRIBUTING.md)了解详情。

### 开发环境设置
```bash
git clone https://github.com/edgeflow/edgeflow.git
cd edgeflow
cargo build
cargo test
```

### 贡献方式
- 🐛 报告Bug和问题
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码补丁
- 🌟 为项目点星支持

## 📄 开源许可

本项目采用MIT和Apache 2.0双许可证 - 详见[LICENSE](LICENSE)文件。

## 🙏 致谢

感谢以下开源项目的支持：
- [Pingora](https://github.com/cloudflare/pingora) - 高性能代理框架
- [WasmEdge](https://wasmedge.org/) - WebAssembly运行时
- [Tokio](https://tokio.rs/) - Rust异步运行时
- [Serde](https://serde.rs/) - Rust序列化框架

---

<div align="center">
  <strong>EdgeFlow - 让AI在边缘流动起来</strong><br>
  <em>AI at the Edge, Flow at Scale</em>
</div>
