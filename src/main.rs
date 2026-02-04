use anyhow::Result;
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters, ServerHandler},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ServiceExt,
};
use sysinfo::{Disks, System};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::{Arc, LazyLock};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetSystemInfoRequest {}

static SYSTEM_INFO_SCHEMA: LazyLock<Arc<serde_json::Map<String, serde_json::Value>>> = LazyLock::new(|| {
    let settings = schemars::generate::SchemaSettings::draft07();
    let generator = settings.into_generator();
    let schema = generator.into_root_schema_for::<GetSystemInfoRequest>();
    let mut val = serde_json::to_value(schema).unwrap();
    let obj = val.as_object_mut().unwrap();
    obj.remove("$schema");
    Arc::new(obj.clone())
});

#[derive(Clone)]
struct SysUtils {
    tool_router: ToolRouter<Self>,
}

fn collect_system_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut report = String::new();

    report.push_str("System Information Report\n");
    report.push_str("=========================\n\n");

    // System name and kernel
    report.push_str(&format!("System Name:      {}\n", System::name().unwrap_or_else(|| "<unknown>".to_string())));
    report.push_str(&format!("Kernel Version:   {}\n", System::kernel_version().unwrap_or_else(|| "<unknown>".to_string())));
    report.push_str(&format!("OS Version:       {}\n", System::os_version().unwrap_or_else(|| "<unknown>".to_string())));
    report.push_str(&format!("Host Name:        {}\n", System::host_name().unwrap_or_else(|| "<unknown>".to_string())));
    
    report.push_str("\nCPU Information\n");
    report.push_str("---------------\n");
    report.push_str(&format!("Number of Cores:  {}\n", sys.cpus().len()));
    
    report.push_str("\nMemory Information\n");
    report.push_str("------------------\n");
    report.push_str(&format!("Total Memory:     {} MB\n", sys.total_memory() / 1024 / 1024));
    report.push_str(&format!("Used Memory:      {} MB\n", sys.used_memory() / 1024 / 1024));
    report.push_str(&format!("Total Swap:       {} MB\n", sys.total_swap() / 1024 / 1024));
    report.push_str(&format!("Used Swap:        {} MB\n", sys.used_swap() / 1024 / 1024));

    report.push_str("\nDisk Information\n");
    report.push_str("----------------\n");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        report.push_str(&format!("Name:             {:?}\n", disk.name()));
        report.push_str(&format!("File System:      {:?}\n", disk.file_system()));
        report.push_str(&format!("Total Space:      {} GB\n", disk.total_space() / 1024 / 1024 / 1024));
        report.push_str(&format!("Available Space:  {} GB\n", disk.available_space() / 1024 / 1024 / 1024));
        report.push_str("---\n");
    }

    report
}

#[tool_router]
impl SysUtils {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get a detailed system information report including kernel, cores, memory, and disk usage.", input_schema = "SYSTEM_INFO_SCHEMA.clone()")]
    async fn get_system_info(
        &self,
        _params: Parameters<GetSystemInfoRequest>,
    ) -> String {
        collect_system_info()
    }
}

#[tool_handler]
impl ServerHandler for SysUtils {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A system utilities MCP that provides detailed system information.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check for CLI arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "info" {
        println!("{}", collect_system_info());
        return Ok(())
    }

    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sysutils_rust=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .json(),
        )
        .init();

    tracing::info!("MCP Starting server on stdio");

    let service = SysUtils::new();
    let transport = rmcp::transport::stdio();
    let server = service.serve(transport).await?;
    server.waiting().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_generation() {
        println!("SCHEMA: {}", serde_json::to_string_pretty(&*SYSTEM_INFO_SCHEMA).unwrap());
    }
}
