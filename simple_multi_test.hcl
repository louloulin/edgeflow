# EdgeFlow简化多上游测试配置
service_name = "edgeflow"
worker_threads = 4
daemon = false

# 服务器配置 - 混合架构
server {
  https_address = "0.0.0.0:8999"
  http_address = "0.0.0.0:8080"
  https_proxy_address = "0.0.0.0:8443"
  enable_tls = false
}

# 简单的多上游路由配置
routes = [
  {
    host = "loadbalance.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3001 },
      { ip = "127.0.0.1", port = 3002 },
      { ip = "127.0.0.1", port = 3003 }
    ]
  },
  {
    host = "api.test"
    upstreams = [
      { ip = "127.0.0.1", port = 3004 }
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
