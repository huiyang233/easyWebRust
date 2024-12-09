import {request} from '@/utils'

export default {
    create: (data) => request.post('/black_list', data),
    read: (params = {}) => request.get('/black_list', { params }),
    update: (data) => request.patch(`/black_list/${data.id}`, data),
    delete: (ip) => request.delete(`/black_list/${ip}`),
}