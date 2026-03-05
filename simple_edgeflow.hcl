# EdgeFlow简单测试配置
service_name = "edgeflow"
worker_threads = 2

# 服务器配置
server {
  https_address = "0.0.0.0:8443"
  http_address = "0.0.0.0:8080"
}

# 简单路由配置
routes = [
  {
    host = "localhost"
    upstreams = [
      {
        ip = "httpbin.org"
        port = 80
      }
    ]
  }
]

# Let's Encrypt配置
lets_encrypt {
  email = "test@edgeflow.ai"
  enabled = false
  staging = true
}

# 日志配置
logging {
  enabled = true
  level = "info"
  access_logs_enabled = true
  error_logs_enabled = true
  format = "pretty"
}
