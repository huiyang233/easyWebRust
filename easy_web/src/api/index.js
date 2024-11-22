/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/04 22:50:38
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import {request} from '@/utils'

export default {
  logout: () => request.post('/login/logout'),
  uploadImage: (image) => {
    const formData = new FormData();
    formData.append('file', image);
    return request.post('/file/image',formData,{
      headers: {'Content-Type': 'multipart/form-daka'}
    })
  },
}
