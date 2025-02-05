use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

/// 初始化定时任务
pub async fn init_scheduler() {
    let mut sched = JobScheduler::new().await.unwrap();
    // 加到这里面去
    sched.add(
        Job::new("1/10 * * * * *", |_, _| print_log_task()).unwrap()
    ).await.unwrap();

    sched.start().await.unwrap();
}
// 下面实现定时任务

fn print_log_task() {
    // 函数实现
    info!("print_log_task")
}