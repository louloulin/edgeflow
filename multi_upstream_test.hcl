# EdgeFlow多上游服务测试配置
service_name = "edgeflow"
worker_threads = 4
daemon = false

# 服务器配置 - 混合架构
server {
  # 管理接口 (EdgeFlow管理API)
  https_address = "0.0.0.0:8999"
  
  # HTTP代理服务 (Let's Encrypt挑战和重定向)
  http_address = "0.0.0.0:8080"
  
  # HTTPS代理服务 (主要代理功能)
  https_proxy_address = "0.0.0.0:8443"
  
  # TLS配置 (暂时禁用)
  enable_tls = false
}

# 多路由配置 - 测试不同的负载均衡场景
routes = [
  # 1. 负载均衡测试 - 3个服务器轮询
  {
    host = "loadbalance.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3001, weight = 1 },
      { ip = "127.0.0.1", port = 3002, weight = 1 },
      { ip = "127.0.0.1", port = 3003, weight = 1 }
    ]
    plugins = [
      { name = "request_id" },
      { name = "performance_analyzer" }
    ]
  },
  
  # 2. 权重负载均衡测试 - 不同权重分配
  {
    host = "weighted.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3001, weight = 3 },
      { ip = "127.0.0.1", port = 3002, weight = 2 },
      { ip = "127.0.0.1", port = 3003, weight = 1 }
    ]
    plugins = [
      { name = "request_id" }
    ]
  },
  
  # 3. API服务器测试
  {
    host = "api.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3004, weight = 1 }
    ]
    plugins = [
      { name = "request_id" },
      { name = "ai_security" }
    ]
  },
  
  # 4. AI服务器测试 - 带AI插件
  {
    host = "ai.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3005, weight = 1 }
    ]
    plugins = [
      { name = "request_id" },
      { name = "ai_security" },
      { name = "llm_router" },
      { name = "prompt_transform" }
    ]
  },
  
  # 5. 高可用测试 - 主备模式
  {
    host = "ha.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3001, weight = 2 },
      { ip = "127.0.0.1", port = 3002, weight = 1 }
    ]
    plugins = [
      { name = "request_id" },
      { name = "performance_analyzer" }
    ]
  },
  
  # 6. 外部服务测试 - httpbin.org
  {
    host = "external.test"
    upstreams = [
      { ip = "httpbin.org", port = 80, weight = 1 }
    ]
    plugins = [
      { name = "request_id" }
    ]
  }
]

# Let's Encrypt配置
lets_encrypt {
  email = "admin@testdomain.com"
  enabled = false
  staging = true
}

# 日志配置
logging {
  enabled = true
  level = "debug"
  access_logs_enabled = true
  error_logs_enabled = true
  format = "pretty"
}
