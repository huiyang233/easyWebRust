<template>
    <AppPage>
        <n-grid x-gap="12" :cols="14">
          
            <n-grid-item :span='10'>
                <n-card title="修改产品信息">
            <n-form ref="modalFormRef" label-placement="left" label-align="left" :label-width="100" :model="editData">
                <n-form-item label="标题" path="name" :rule="{
        required: true,
        message: '请输入标题',
        trigger: ['input', 'blur'],
    }">
                    <n-input v-model:value="editData.name" />
                </n-form-item>
                <n-form-item label="状态" path="status">
                    <n-select v-model:value="editData.status" :options="options" />

                </n-form-item>


                <n-form-item label="主图" path="mainImage" :rule="{
        required: true,
        message: '请输入标题',
        trigger: ['input', 'blur'],
    }">
                    <n-upload max='1' action="https://www.mocky.io/v2/5e4bafc63100007100d8b70f"
                        :default-file-list="previewFileList" list-type="image-card" @preview="handlePreview" />
                    <n-modal v-model:show="showModal" preset="card" style="width: 600px" title="一张很酷的图片">
                        <img :src="previewImageUrl" style="width: 100%">
                    </n-modal>
                </n-form-item>


                <n-form-item label="内容" path="content" :rule="{
        required: true,
        message: '请输入标题',
        trigger: ['input', 'blur'],
    }">
                    <div style="border: 1px solid #ccc">
                        <Toolbar style="border-bottom: 1px solid #ccc" :editor="editorRef"
                            :defaultConfig="toolbarConfig" :mode="mode" />
                        <Editor style="height: 300px; overflow-y: hidden;" v-model="editData.description"
                            :defaultConfig="editorConfig" :mode="mode" @onCreated="handleCreated" />
                    </div>
                </n-form-item>


            </n-form>

        
        </n-card>
        </n-grid-item>
   

        <n-grid-item :span="4">
            <n-card title="操作">
            <div class="flex justify-end">
                <n-button :loading="loading" @click="handleSubmit()" type="primary" class="ml-20">
                    保存
                </n-button>
            </div>
            </n-card>
            
        </n-grid-item>
        </n-grid>
        

    </AppPage>
</template>
<script setup>

import { onBeforeUnmount, ref, shallowRef, onMounted } from 'vue'
import { Editor, Toolbar } from '@wangeditor/editor-for-vue'
import api from './api'
const options = ref(
    [{
        label: '未发布',
        value: 0
    },
    {
        label: '已发布',
        value: 1
    },
    {
        label: '已下架',
        value: 2
    }]
)

const showModalRef = ref(false);
const previewImageUrlRef = ref("");
function handlePreview(file) {
    const { url } = file;
    previewImageUrlRef.value = url;
    showModalRef.value = true;
}

const router = useRouter()
const editData = ref({})
const loading = ref(false)
const editorRef = shallowRef()

async function readData() {
    const id = router.currentRoute.value.query
    const { data } = await api.readId(id)
    console.log(data)
    editData.value = data;

}
readData()
onMounted(() => {

})

// 内容 HTML

const toolbarConfig = {}
const editorConfig = { placeholder: '请输入内容...' }
onBeforeUnmount(() => {
    const editor = editorRef.value
    if (editor == null) return
    editor.destroy()
})
const handleCreated = (editor) => {
    editorRef.value = editor // 记录 editor 实例，重要！
}
// 获取Url上的id




</script>
<style lang="">

</style>