use crate::model::result::{Http, WebResult};
use crate::RB;
use rbatis::RBatis;
use rbs::Value;
use std::ops::Deref;

pub struct SysReport;

impl SysReport {
    // 查询七天内用户登录统计
    pub async fn select_login_count_by_seven_day() -> Http<Value> {
        let id = SysReport::select_login_count_by_seven_day_(RB.deref()).await?;
        Ok(WebResult::success(id))
    }

    pub async fn select_login_count_by_seven_day_(rb: &RBatis) -> rbatis::Result<Value> {

        {} use rbatis::executor::Executor;
        let r = rb.query(&"WITH date_series AS (
    SELECT CURDATE() - INTERVAL (n - 1) DAY AS login_date
    FROM (
             SELECT 1 n UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5 UNION ALL SELECT 6 UNION ALL SELECT 7
         ) numbers
)
SELECT
    ds.login_date,
    COALESCE(sl.login_count, 0) AS login_count
FROM
    date_series ds
        LEFT JOIN (
        SELECT
            DATE(create_time) AS login_date,
            COUNT(*) AS login_count
        FROM
            sys_log
        WHERE
            create_time >= DATE_SUB(CURDATE(), INTERVAL 7 DAY)
        GROUP BY
            DATE(create_time)
    ) sl ON ds.login_date = sl.login_date
ORDER BY
    ds.login_date", Vec::new()).await?;
        Ok(r)
    }
}