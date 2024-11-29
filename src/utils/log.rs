use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn log_init(){
    // 初始化日志
    let timer = tracing_subscriber::fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S %:z".into()); // 自定义格式
    let file_appender = tracing_appender::rolling::daily("./log", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                // .pretty() // 选择输出格式为 pretty
                .with_thread_ids(true)
                .with_timer(timer)
                .with_file(false) // 输出附带文件路径
                .with_line_number(true) // 输出附带行号
                .with_ansi(false) // 输出附带 ansi 颜色输出与特殊符号
                .with_writer( tracing_subscriber::fmt::writer::Tee::new(
                    || std::io::stdout(),
                    non_blocking,
                ))
                .with_target(true), // 输出附带 target
        )
        .init();
}