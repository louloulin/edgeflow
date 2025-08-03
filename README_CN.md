<div align="center">

# EdgeFlow

**🚀 下一代边缘AI基础设施平台**

*让AI在边缘流动起来*

[![GitHub Release](https://img.shields.io/github/v/release/louloulin/edgeflow?style=for-the-badge)](https://github.com/louloulin/edgeflow/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?style=for-the-badge)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/actions/workflow/status/louloulin/edgeflow/test.yml?style=for-the-badge)](https://github.com/louloulin/edgeflow/actions)

[🚀 快速开始](#-快速开始) • [📖 文档](docs/) • [🤝 贡献](CONTRIBUTING.md) • [💬 社区](#-社区) • [🌐 English](README.md)

</div>

---

## 🎯 什么是EdgeFlow？

EdgeFlow是一个**高性能边缘AI网关和推理平台**，将AI能力直接带到边缘。基于Rust构建，由Cloudflare的Pingora驱动，EdgeFlow提供企业级AI基础设施，具有亚10毫秒延迟、WebAssembly插件生态系统和全面的可观测性。

### 🌟 核心亮点

- **🚀 超高性能**: 10万+QPS，<10ms AI推理延迟
- **🤖 AI原生设计**: 专为LLM和边缘AI工作负载构建
- **🔌 可扩展插件系统**: 16个内置插件 + WebAssembly运行时
- **🛡️ 企业级安全**: OAuth2、JWT、多租户隔离、AI安全
- **📊 全面可观测性**: 实时指标、AI分析、Prometheus集成
- **🌐 边缘优先**: 为边缘部署优化，支持离线能力

---

## ✨ 核心特性

### 🤖 **AI原生架构**
- **边缘推理**: 本地AI模型推理，延迟<10ms
- **智能路由**: 基于AI能力的智能请求路由
- **模型管理**: 自动模型发现、缓存和负载均衡
- **多模态支持**: 文本、图像、音频、视频统一处理
- **LLM聚合**: 多模型响应聚合和优化

### ⚡ **高性能网关**
- **极致性能**: Rust + Pingora基础，支持10万QPS
- **WebAssembly运行时**: 安全的WASM插件生态系统
- **自动HTTPS**: Let's Encrypt自动证书管理
- **智能负载均衡**: 智能负载均衡和故障转移
- **语义缓存**: AI优化的语义哈希缓存

### 🌐 **边缘优先设计**
- **边缘优化**: 专为边缘计算场景构建
- **离线能力**: 完整的离线AI推理支持
- **资源高效**: 相比Docker减少50%资源使用
- **即时启动**: 毫秒级冷启动时间
- **分布式**: 多区域部署和集群支持

### 🔌 **可扩展插件系统**
- **16个内置插件**: 认证、缓存、监控、AI处理
- **WASM插件**: 多语言插件开发支持
- **热重载**: 无需重启即可更新插件
- **插件市场**: 社区驱动的插件生态

### 🛡️ **企业级安全**
- **AI安全**: 提示注入检测和内容过滤
- **OAuth2集成**: GitHub、WorkOS和自定义提供商支持
- **零信任**: 端到端加密和访问控制
- **合规性**: GDPR、SOC2和企业合规支持
- **多租户**: 安全的租户隔离和配额管理

### 📊 **全面可观测性**
- **实时监控**: 性能指标和健康检查
- **AI分析**: AI请求/响应模式的深度分析
- **Web仪表板**: 基于浏览器的管理和分析界面
- **Prometheus集成**: 无缝监控系统集成
- **审计日志**: 完整的合规和安全审计跟踪

---

## 🚀 快速开始

### 安装

#### 选项1: 从Cargo安装
```bash
cargo install edgeflow
```

#### 选项2: 从源码构建
```bash
git clone https://github.com/louloulin/edgeflow.git
cd edgeflow
cargo build --release
```

#### 选项3: Docker部署
```bash
docker run -p 80:80 -p 443:443 ghcr.io/louloulin/edgeflow:latest
```

### 基础配置

创建配置文件 `edgeflow.hcl`:
```hcl
# EdgeFlow配置
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

# 或使用默认配置启动
edgeflow start
```

### 验证安装
```bash
# 检查服务健康状态
curl http://localhost/health

# 列出可用的AI模型
curl http://localhost:8080/v1/models

# 测试AI推理
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama2-7b",
    "messages": [{"role": "user", "content": "你好，EdgeFlow！"}]
  }'
```

---

## 📚 文档和资源

### 📖 官方文档
- [📋 快速开始指南](docs/quick-start.md)
- [⚙️ 配置参考](docs/configuration.md)
- [🔌 插件开发](docs/plugins.md)
- [🌐 API文档](docs/api.md)
- [🚀 部署指南](docs/deployment.md)
- [🏗️ 架构概览](docs/architecture.md)

### 🛠️ 示例和教程
- [AI网关设置](examples/ai_gateway_config.hcl)
- [OAuth2认证](examples/oauth2_setup.md)
- [性能优化](docs/performance_optimization.md)
- [插件开发教程](docs/plugin_tutorial.md)

### 🎯 使用场景
- [边缘AI推理](docs/use-cases/edge-inference.md)
- [API网关](docs/use-cases/api-gateway.md)
- [多租户SaaS](docs/use-cases/multi-tenant.md)
- [微服务网格](docs/use-cases/microservices.md)

---

## 🏗️ 架构

EdgeFlow基于现代云原生架构构建，专为边缘AI工作负载设计：

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   客户端应用    │    │   Web仪表板     │    │   管理API      │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │      EdgeFlow网关         │
                    │   (Rust + Pingora核心)   │
                    └─────────────┬─────────────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          │                      │                      │
    ┌─────▼─────┐        ┌───────▼───────┐      ┌───────▼───────┐
    │  插件     │        │   AI引擎      │      │   存储层      │
    │  系统     │        │   (WASM/LLM)  │      │               │
    │ (16个内置)│        │               │      │               │
    └───────────┘        └───────────────┘      └───────────────┘
```

### 核心组件
- **网关核心**: 高性能Rust + Pingora代理
- **插件系统**: 基于WebAssembly的可扩展插件架构
- **AI引擎**: 支持LLM的边缘AI推理
- **存储层**: 分布式缓存和持久化
- **管理API**: 用于配置和监控的RESTful API

---

## 🚀 性能基准

EdgeFlow在所有指标上都提供卓越的性能：

| 指标 | EdgeFlow | 传统网关 | 改进 |
|------|----------|----------|------|
| **吞吐量** | 10万+QPS | 5万QPS | **快2倍** |
| **延迟** | <10ms | 50-100ms | **快5-10倍** |
| **内存使用** | 50MB | 200MB | **高效4倍** |
| **冷启动** | <1ms | 100-500ms | **快100-500倍** |
| **AI推理** | <10ms | 100-1000ms | **快10-100倍** |

*基准测试在AWS c5.2xlarge实例上运行，使用真实工作负载*

---

## 💼 企业版

EdgeFlow为生产部署提供企业级功能和支持：

### 🏢 企业功能
- **24/7技术支持**: 专业支持团队
- **高可用性**: 多区域灾难恢复
- **企业安全**: 高级安全和合规功能
- **定制开发**: 为您的需求定制功能
- **专业培训**: 技术培训和认证

### 📞 联系我们
- **网站**: [edgeflow.ai/enterprise](https://edgeflow.ai/enterprise)
- **邮箱**: enterprise@edgeflow.ai
- **销售**: sales@edgeflow.ai

---

## 🤝 贡献

我们欢迎社区贡献！请查看我们的[贡献指南](CONTRIBUTING.md)了解详情。

### 🛠️ 开发设置
```bash
git clone https://github.com/louloulin/edgeflow.git
cd edgeflow
cargo build
cargo test
```

### 🎯 贡献方式
- 🐛 **报告Bug**: 帮助我们识别和修复问题
- 💡 **功能请求**: 建议新功能和改进
- 📝 **文档**: 改进文档和示例
- 🔧 **代码贡献**: 提交拉取请求
- 🌟 **社区**: 为项目点星并传播

---

## 💬 社区

加入我们不断增长的开发者和用户社区：

- **GitHub**: [louloulin/edgeflow](https://github.com/louloulin/edgeflow)
- **Discord**: [加入我们的Discord](https://discord.gg/edgeflow)
- **微博**: [@EdgeFlowAI](https://weibo.com/EdgeFlowAI)
- **微信群**: 扫描二维码加入

---

## 📄 许可证

本项目采用MIT和Apache 2.0双许可证。详见[LICENSE](LICENSE)。

---

## 🙏 致谢

EdgeFlow建立在巨人的肩膀上。特别感谢：

- **[Pingora](https://github.com/cloudflare/pingora)** - Cloudflare的高性能代理框架
- **[WasmEdge](https://wasmedge.org/)** - 轻量级WebAssembly运行时
- **[Tokio](https://tokio.rs/)** - Rust异步运行时
- **[Serde](https://serde.rs/)** - Rust序列化框架

---

<div align="center">

**EdgeFlow - 让AI在边缘流动起来**

*构建边缘AI基础设施的未来*

[🚀 开始使用](#-快速开始) • [📖 文档](docs/) • [💬 社区](#-社区) • [🤝 贡献](CONTRIBUTING.md)

</div>
