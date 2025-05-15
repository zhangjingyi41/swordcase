<script setup lang="ts">
import { defineProps } from 'vue';
import { TabItem } from '@/stores/app';
const props = defineProps({
    tabItem:{
        type: Object as () => TabItem,
        required: true
    }
})
</script>
<template>
    <el-card footer-class="footer">
        <p>{{ props.tabItem }}</p>
        <template #footer>
            {{ props.tabItem.name }}
        </template>
    </el-card>
</template>
<style scoped>
.footer{
    height: 33%;
}
</style>