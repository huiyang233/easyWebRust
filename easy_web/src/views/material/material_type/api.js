import { request } from '@/utils'

export default {
  create: (data) => request.post('/material_type', data),
  read: (params = {}) => request.get('/material_type', { params }),
  update: (data) => request.patch(`/material_type/${data.id}`, data),
  delete: (id) => request.delete(`/material_type/${id}`),
}