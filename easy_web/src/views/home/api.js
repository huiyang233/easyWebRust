import {request} from '@/utils'

export default {
    getLoginCountData: () => request.get('/report/select_login_count_by_seven_day'),
    getUserCount: () => request.get('/report/select_user_count'),
    getTodayLoginCount: () => request.get('/report/select_login_count_by_today'),
    getTodayActiveUsersCount: () => request.get('/report/select_active_users_by_today'),

}