# EdgeFlow测试配置文件
# 用于验证网关功能

service_name = "edgeflow"
worker_threads = 2

# 服务器配置
server {
  # HTTP服务器配置
  http_address = "0.0.0.0:8080"
  
  # HTTPS服务器配置（可选）
  # https_address = "0.0.0.0:8443"
}

# 基础路由配置
routes = [
  {
    # 测试路由：将请求转发到httpbin.org进行测试
    host = "test.edgeflow.local"
    path = "/"
    upstream = {
      address = "httpbin.org:80"
      protocol = "http"
    }
  },
  {
    # 健康检查路由
    host = "localhost"
    path = "/health"
    upstream = {
      address = "httpbin.org:80"
      protocol = "http"
    }
  }
]

# 插件配置
plugins = [
  {
    name = "edgeflow_request_id"
    enabled = true
    config = {
      header_name = "X-EdgeFlow-Request-ID"
    }
  }
]

# 日志配置
logging {
  level = "info"
  format = "json"
}

# 监控配置
monitoring {
  enabled = true
  metrics_port = 9090
}
