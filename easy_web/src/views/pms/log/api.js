import { request } from '@/utils'

export default {
  read: (params = {}) => request.get('/sys_log', { params }),
}