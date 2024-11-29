import {request} from '@/utils'

export default {
    getLoginCountData: () => request.get('/report'),
}