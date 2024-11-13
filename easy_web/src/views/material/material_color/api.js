import { request } from '@/utils'

export default {
  create: (data) => request.post('/material_color', data),
  read: (params = {}) => request.get('/material_color', { params }),
  update: (data) => request.patch(`/material_color/${data.id}`, data),
  delete: (id) => request.delete(`/material_color/${id}`),
}