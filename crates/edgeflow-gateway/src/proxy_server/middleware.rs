use std::collections::HashMap;
use std::sync::Arc;

use pingora::Result;
use tracing::{debug, warn};

use crate::plugins::core::{Plugin, PluginStep};
use crate::plugins::executor;

/// Executes the response plugins using the new plugin executor
pub async fn execute_response_plugins(
    session: &mut pingora::proxy::Session,
    ctx: &mut crate::proxy_server::https_proxy::RouterContext,
) -> Result<()> {
    debug!("Executing response plugins using new executor");

    // Get plugins from the plugin manager
    let plugins = get_active_plugins_for_route(ctx).await;

    // Execute response plugins
    match executor::execute_plugins(PluginStep::Response, session, ctx, &plugins).await {
        Ok((handled, response)) => {
            if handled {
                debug!("Response plugins handled the response");
                if let Some(_resp) = response {
                    // Note: HttpResponse to ResponseHeader conversion would be needed here
                    // For now, we just log that a response was handled
                    debug!("Plugin provided custom response");
                }
            }
        },
        Err(e) => {
            warn!("Error executing response plugins: {}", e);
        }
    }

    // Execute logging plugins
    match executor::execute_plugins(PluginStep::Log, session, ctx, &plugins).await {
        Ok((_, _)) => {
            debug!("Logging plugins executed successfully");
        },
        Err(e) => {
            warn!("Error executing logging plugins: {}", e);
        }
    }

    Ok(())
}

/*
    for (name, value) in ctx.route_container.plugins.clone() {
        match name.as_str() {
            "oauth2" => {
                // use crate::plugins::MiddlewarePlugin;

                // if crate::plugins::PLUGINS
                //     .oauth2
                //     .response_filter(session, ctx, &value)
                //     .await
                //     .is_ok_and(|v| v)
                // {
                //     return Ok(());
                // }
            }
            "request_id" => continue,
            // AI Gateway plugins
            "llm_router" => {
                // if crate::plugins::PLUGINS
                //     .llm_router
                //     .response_filter(session, ctx, &value)
                //     .await
                //     .is_ok_and(|v| v)
                // {
                //     return Ok(());
                // }
            },
            "prompt_transform" => {
                // if crate::plugins::PLUGINS
                //     .prompt_transform
                //     .response_filter(session, ctx, &value)
                //     .await
                //     .is_ok_and(|v| v)
                // {
                //     return Ok(());
                // }
            },
            "ai_security" => {
                // if crate::plugins::PLUGINS
                //     .ai_security
                //     .response_filter(session, ctx, &value)
                //     .await
                //     .is_ok_and(|v| v)
                // {
                //     return Ok(());
                // }
            },
            "llm_aggregator" => {
                // if crate::plugins::PLUGINS
                //     .llm_aggregator
                //     .response_filter(session, ctx, &value)
                //     .await
                //     .is_ok_and(|v| v)
                // {
                //     return Ok(());
                // }
            },
            _ => {}
        }
    }
    */

/// Executes the request plugins using the new plugin executor
pub async fn execute_request_plugins(
    session: &mut pingora::proxy::Session,
    ctx: &mut crate::proxy_server::https_proxy::RouterContext,
    plugins: &HashMap<String, crate::config::RoutePlugin>,
) -> Result<bool> {
    debug!("Executing request plugins for {} plugins", plugins.len());

    // Get active plugins for this route
    let active_plugins = get_active_plugins_for_route(ctx).await;

    // Execute early request plugins first
    match executor::execute_plugins(PluginStep::EarlyRequest, session, ctx, &active_plugins).await {
        Ok((handled, response)) => {
            if handled {
                debug!("Early request plugins handled the request");
                if let Some(_resp) = response {
                    // Note: HttpResponse to ResponseHeader conversion would be needed here
                    debug!("Plugin provided custom response");
                }
                return Ok(true);
            }
        },
        Err(e) => {
            warn!("Error executing early request plugins: {}", e);
        }
    }

    // Execute standard request plugins
    match executor::execute_plugins(PluginStep::Request, session, ctx, &active_plugins).await {
        Ok((handled, response)) => {
            if handled {
                debug!("Request plugins handled the request");
                if let Some(_resp) = response {
                    // Note: HttpResponse to ResponseHeader conversion would be needed here
                    debug!("Plugin provided custom response");
                }
                return Ok(true);
            }
        },
        Err(e) => {
            warn!("Error executing request plugins: {}", e);
        }
    }

    // 执行插件逻辑
    for (name, plugin_config) in plugins {
        tracing::debug!("Processing plugin: {}", name);

        match name.as_str() {
            "oauth2" => {
                tracing::debug!("Executing OAuth2 plugin");
                // 基础OAuth2验证逻辑
                let req_header = session.req_header_mut();
                    if let Some(auth_header) = req_header.headers.get("authorization") {
                        if let Ok(auth_str) = auth_header.to_str() {
                            if auth_str.starts_with("Bearer ") {
                                tracing::debug!("OAuth2: Valid Bearer token format");
                                // 这里可以添加更复杂的token验证逻辑
                            } else {
                                tracing::warn!("OAuth2: Invalid authorization header format");
                                return Ok(true); // 阻止请求
                            }
                        }
                    } else {
                        tracing::warn!("OAuth2: Missing authorization header");
                        return Ok(true); // 阻止请求
                    }
            }
            "request_id" => {
                tracing::debug!("Executing Request ID plugin");
                // 添加请求ID
                let req_header = session.req_header_mut();
                let request_id = uuid::Uuid::new_v4().to_string();
                ctx.request_id = request_id.clone();
                if let Ok(header_value) = http::HeaderValue::from_str(&request_id) {
                    req_header.insert_header("X-Request-ID", &header_value).ok();
                    tracing::debug!("Added request ID: {}", request_id);
                }
            }
            "basic_auth" => {
                tracing::debug!("Executing Basic Auth plugin");
                // 基础认证逻辑
                let req_header = session.req_header_mut();
                if let Some(auth_header) = req_header.headers.get("authorization") {
                    if let Ok(auth_str) = auth_header.to_str() {
                        if auth_str.starts_with("Basic ") {
                            tracing::debug!("Basic Auth: Valid Basic auth format");
                            // 这里可以添加更复杂的认证逻辑
                        } else {
                            tracing::warn!("Basic Auth: Invalid authorization header format");
                            return Ok(true); // 阻止请求
                        }
                    }
                } else {
                    tracing::warn!("Basic Auth: Missing authorization header");
                    return Ok(true); // 阻止请求
                }
            }
            // AI Gateway plugins
            "llm_router" => {
                tracing::debug!("Executing LLM Router plugin");
                // LLM路由逻辑
                let req_header = session.req_header_mut();
                // 检查是否是AI请求
                if req_header.uri.path().contains("/v1/chat/completions") ||
                   req_header.uri.path().contains("/v1/completions") {
                    tracing::debug!("LLM Router: Detected AI request");
                    // 这里可以添加模型路由逻辑
                    req_header.insert_header("X-AI-Route", "llm").ok();
                }
            },
            "prompt_transform" => {
                tracing::debug!("Executing Prompt Transform plugin");
                // 提示转换逻辑
                let req_header = session.req_header_mut();
                if req_header.uri.path().contains("/v1/chat/completions") {
                    tracing::debug!("Prompt Transform: Processing AI request");
                    // 这里可以添加提示增强逻辑
                    req_header.insert_header("X-Prompt-Enhanced", "true").ok();
                }
            },
            "ai_security" => {
                tracing::debug!("Executing AI Security plugin");
                // AI安全检查逻辑
                let req_header = session.req_header_mut();
                if req_header.uri.path().contains("/v1/chat/completions") {
                    tracing::debug!("AI Security: Scanning for malicious content");
                    // 这里可以添加恶意内容检测逻辑
                    // 简单示例：检查危险关键词
                    if req_header.uri.query().unwrap_or("").contains("ignore previous instructions") {
                        tracing::warn!("AI Security: Detected potential prompt injection");
                        return Ok(true); // 阻止请求
                    }
                    req_header.insert_header("X-AI-Security-Checked", "true").ok();
                }
            },
            "llm_aggregator" => {
                tracing::debug!("Executing LLM Aggregator plugin");
                // LLM聚合逻辑
                let req_header = session.req_header_mut();
                req_header.insert_header("X-LLM-Aggregator", "enabled").ok();
            },
            "vector_db" => {
                tracing::debug!("Executing Vector DB plugin");
                // 向量数据库逻辑
                let req_header = session.req_header_mut();
                req_header.insert_header("X-Vector-DB", "enabled").ok();
            },
            "ai_analytics" => {
                tracing::debug!("Executing AI Analytics plugin");
                // AI分析逻辑
                let req_header = session.req_header_mut();
                req_header.insert_header("X-AI-Analytics", "enabled").ok();
            },
            "ai_request_builder" => {
                tracing::debug!("Executing AI Request Builder plugin");
                // AI请求构建逻辑
                let req_header = session.req_header_mut();
                req_header.insert_header("X-AI-Request-Builder", "enabled").ok();
            },
            "prompt_debugger" => {
                tracing::debug!("Executing Prompt Debugger plugin");
                // 提示调试逻辑
                let req_header = session.req_header_mut();
                req_header.insert_header("X-Prompt-Debugger", "enabled").ok();
            },
            "performance_analyzer" => {
                tracing::debug!("Executing Performance Analyzer plugin");
                // 性能分析逻辑
                let req_header = session.req_header_mut();
                req_header.insert_header("X-Performance-Analyzer", "enabled").ok();
            },
            _ => {
                tracing::debug!("Unknown plugin: {}", name);
            }
        }
    }

    tracing::debug!("All request plugins executed successfully");
    Ok(false) // 不阻止请求
}

/// Executes the upstream request plugins
#[allow(unused_variables)] // Temporarily allow unused vars until logic is replaced
pub async fn execute_upstream_request_plugins(
    session: &mut pingora::proxy::Session,
    upstream_request: &mut pingora::http::RequestHeader,
    ctx: &mut crate::proxy_server::https_proxy::RouterContext,
) -> Result<()> {
    tracing::debug!("Executing upstream request plugins (Old middleware - logic needs update)");
    // TODO: Replace this entire function body with calls to executor::execute_plugins for PluginStep::ProxyUpstream
    /*
    for (name, value) in ctx.route_container.plugins.clone() {
        match name.as_str() {
            "request_id" => {
                // crate::plugins::PLUGINS
                //     .request_id
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            }
            // AI Gateway plugins
            "llm_router" => {
                // crate::plugins::PLUGINS
                //     .llm_router
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "prompt_transform" => {
                // crate::plugins::PLUGINS
                //     .prompt_transform
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "ai_security" => {
                // crate::plugins::PLUGINS
                //     .ai_security
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "llm_aggregator" => {
                // crate::plugins::PLUGINS
                //     .llm_aggregator
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "vector_db" => {
                // crate::plugins::PLUGINS
                //     .vector_db
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "ai_analytics" => {
                // crate::plugins::PLUGINS
                //     .ai_analytics
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "ai_request_builder" => {
                // crate::plugins::PLUGINS
                //     .ai_request_builder
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "prompt_debugger" => {
                // crate::plugins::PLUGINS
                //     .prompt_debugger
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            "performance_analyzer" => {
                // crate::plugins::PLUGINS
                //     .performance_analyzer
                //     .upstream_request_filter(session, upstream_request, ctx)
                //     .await
                //     .ok();
            },
            _ => {}
        }
    }
    */
    Ok(())
}

/// Executes the upstream response plugins
#[allow(unused_variables)] // Temporarily allow unused vars until logic is replaced
pub fn execute_upstream_response_plugins(
    session: &mut pingora::proxy::Session,
    upstream_response: &mut pingora::http::ResponseHeader,
    ctx: &mut crate::proxy_server::https_proxy::RouterContext,
) {
    tracing::debug!("Executing upstream response plugins (Old middleware - logic needs update)");
    // TODO: Replace this entire function body with calls to executor::execute_plugins for PluginStep::Response
    /*
    for (name, value) in ctx.route_container.plugins.clone() {
        match name.as_str() {
            "oauth2" => {
                // use crate::plugins::MiddlewarePlugin;
                // let _ = crate::plugins::PLUGINS
                //     .oauth2
                //     .response_filter(session, ctx, &value);
            }
            "request_id" => {
                // let _ = crate::plugins::PLUGINS
                //     .request_id
                //     .response_filter(session, ctx, &value);
            }
            // AI Gateway plugins
            "llm_router" => {
                // let _ = crate::plugins::PLUGINS
                //     .llm_router
                //     .response_filter(session, ctx, &value);
            },
            "prompt_transform" => {
                // let _ = crate::plugins::PLUGINS
                //     .prompt_transform
                //     .response_filter(session, ctx, &value);
            },
            "ai_security" => {
                // let _ = crate::plugins::PLUGINS
                //     .ai_security
                //     .response_filter(session, ctx, &value);
            },
            "llm_aggregator" => {
                // let _ = crate::plugins::PLUGINS
                //     .llm_aggregator
                //     .response_filter(session, ctx, &value);
            },
            _ => {}
        }
    }
    */
}

/// Get active plugins for the current route
async fn get_active_plugins_for_route(
    ctx: &crate::proxy_server::https_proxy::RouterContext,
) -> Vec<Arc<dyn Plugin>> {
    let mut plugins = Vec::new();

    // Get plugins from the global plugin registry
    for (plugin_name, plugin_config) in &ctx.route_container.plugins {
        if let Some(plugin) = get_plugin_instance(plugin_name, plugin_config).await {
            debug!("Adding plugin '{}' for route", plugin_name);
            plugins.push(plugin);
        } else {
            warn!("Plugin '{}' not found or failed to initialize", plugin_name);
        }
    }

    plugins
}

/// Get a plugin instance by name and configuration
async fn get_plugin_instance(
    plugin_name: &str,
    _plugin_config: &crate::config::RoutePlugin,
) -> Option<Arc<dyn Plugin>> {
    match plugin_name {
        "request_id" => {
            // Create request_id plugin instance
            Some(create_request_id_plugin().await)
        },
        "ai_security" => {
            // Create AI security plugin instance
            Some(create_ai_security_plugin().await)
        },
        "llm_router" => {
            // Create LLM router plugin instance
            Some(create_llm_router_plugin().await)
        },
        "performance_analyzer" => {
            // Create performance analyzer plugin instance
            Some(create_performance_analyzer_plugin().await)
        },
        "prompt_transform" => {
            // Create prompt transform plugin instance
            Some(create_prompt_transform_plugin().await)
        },
        _ => {
            debug!("Unknown plugin: {}", plugin_name);
            None
        }
    }
}

/// Create request_id plugin instance
async fn create_request_id_plugin() -> Arc<dyn Plugin> {
    use crate::plugins::request_id::RequestId;
    Arc::new(RequestId::new())
}

/// Create AI security plugin instance
async fn create_ai_security_plugin() -> Arc<dyn Plugin> {
    use crate::plugins::ai_security::AiSecurity;
    Arc::new(AiSecurity::new())
}

/// Create LLM router plugin instance
async fn create_llm_router_plugin() -> Arc<dyn Plugin> {
    use crate::plugins::llm_router::LlmRouter;
    Arc::new(LlmRouter::new())
}

/// Create performance analyzer plugin instance
async fn create_performance_analyzer_plugin() -> Arc<dyn Plugin> {
    use crate::plugins::performance_analyzer::PerformanceAnalyzer;
    Arc::new(PerformanceAnalyzer::new())
}

/// Create prompt transform plugin instance
async fn create_prompt_transform_plugin() -> Arc<dyn Plugin> {
    use crate::plugins::prompt_transform::PromptTransformer;
    Arc::new(PromptTransformer::new())
}
