import { request } from '@/utils'

export default {
  create: (data) => request.post('/goods_info', data),
  read: (params = {}) => request.get('/goods_info', { params }),
  readId: (params = {}) => request.get(`/goods_info/${params.id}`),
  update: (data) => request.patch(`/goods_info/${data.id}`, data),
  delete: (id) => request.delete(`/goods_info/${id}`),
}