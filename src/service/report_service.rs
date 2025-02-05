use crate::model::result::{Http, WebResult};
use crate::model::user::SysUser;


use crate::utils::db::QueryBuilder;
use serde_json::Value;

pub struct SysReport;

impl SysReport {
    // 查询七天内用户登录统计
    pub async fn select_login_count_by_seven_day() -> Http<Value> {
        let builder = QueryBuilder::<Value>::new_sql("
        WITH date_series AS (
            SELECT CURRENT_DATE - INTERVAL '1 day' * (n - 1) AS login_date
            FROM generate_series(1, 7) AS n
        )
        SELECT
            TO_CHAR(ds.login_date, 'YYYY-MM-DD') AS login_date,
            COALESCE(sl.login_count, 0) AS login_count
        FROM
            date_series ds
                LEFT JOIN (
                SELECT
                    create_time::date AS login_date,
                    COUNT(*) AS login_count
                FROM
                    sys_log
                WHERE
                    create_time >= CURRENT_DATE - INTERVAL '7 days'
                GROUP BY
                    create_time::date
            ) sl ON ds.login_date = sl.login_date
        ORDER BY
            ds.login_date;
        ").fetch_value().await?;
        Ok(WebResult::success(builder))
    }

    // 当前用户数量
    pub async fn select_user_count() -> Http<i64> {
        let id = SysUser::get_count().await?;
        Ok(WebResult::success(id))
    }
    // 今日登录数
    pub async fn select_login_count_by_today() -> Http<i64> {
        let count = QueryBuilder::<i64>::new_sql("SELECT COUNT(*) FROM sys_log WHERE create_time::date = CURRENT_DATE")
            .scalar_fetch_one().await?;
        Ok(WebResult::success(count))
    }


    pub async fn select_active_users() -> Http<i64> {
        let count = QueryBuilder::<i64>::new_sql("SELECT COUNT(DISTINCT user_id) FROM request_log WHERE create_time::date = CURRENT_DATE  AND user_id IS NOT NULL")
            .scalar_fetch_one().await?;
        Ok(WebResult::success(count))
    }
}