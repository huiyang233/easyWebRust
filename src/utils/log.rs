use std::sync::Arc;
use tracing::info;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn log_init(non_blocking: NonBlocking){
    // 初始化日志
    let timer = tracing_subscriber::fmt::time::ChronoLocal::new("%F %H:%M:%S%.6f".into()); // 自定义格式
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer()
                // .pretty() // 选择输出格式为 pretty
                .with_thread_ids(true)
                .with_level(true)
                .with_timer(timer)
                .with_file(false) // 输出附带文件路径
                .with_line_number(true) // 输出附带行号
                .with_ansi(false) // 输出附带 ansi 颜色输出与特殊符号
                .with_target(true) // 输出附带 target
                .with_writer( tracing_subscriber::fmt::writer::Tee::new(
                    || std::io::stdout(),
                    non_blocking,
                ))
        )
        .init();
    info!("Logging initialized successfully");
}