<script setup lang="ts">
    import { faFolder } from '@fortawesome/free-regular-svg-icons';
    import type { UnwrapNestedRefs } from 'vue';
    import type { VaultFile } from '~/models/vaults';
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

    const IMAGE_PREVIEW_FILE_EXTENSIONS = ['png', 'apng', 'avif', 'gif', 'jpg', 'jpeg', 'jfif', 'webp'];
    
    const config = useRuntimeConfig();

    const props = defineProps<{
        vaultId: string,
        file: VaultFile,
    }>();

    const isPreviewableImage = computed(() => IMAGE_PREVIEW_FILE_EXTENSIONS.includes(props.file.name.split('.').at(-1) || ''));

    const previewData = ref<UnwrapNestedRefs<PromiseState<string>>>();

    const containerElement = ref<HTMLDivElement>();
    const previewImageElement = ref<HTMLImageElement>();

    watch(() => props.file.id, async () => {
        if (isPreviewableImage.value) {
            previewData.value = reactive(usePromise($fetch<Blob>(
                `${config.public.apiBase}/vaults/${props.vaultId}/files/${props.file.id}`,
                { 
                    headers: { 'Authorization': `Bearer ${await auth.getAccessToken()}` },
                    responseType: 'blob',
                },
            ).then(blob => URL.createObjectURL(blob))));
        } else {
            previewData.value = undefined;
        }
    });

    onUnmounted(() => {
        if (previewData?.value?.value) {
            URL.revokeObjectURL(previewData.value.value);
        }
    });
</script>

<template>
    <div ref="containerElement" class="flex flex-col items-center py-4 outlined h-fit overflow-hidden sticky top-5 max-h-[87vh]">
        <div class="flex flex-col items-center w-full px-2">
            <h2 class="text-xl font-semibold text-center w-[95%] break-all">
                {{ file.name }}
            </h2>
            <span class="text-center">
                <template v-if="file.created_at !== null">
                    Created {{ formatDuration((Number(new Date()) - Number(new Date(file.created_at))) / 1000) }} ago
                </template>

                <template v-if="file.size !== null && file.file_type === 'file'">
                    &middot; {{ formatFileSize(file.size) }}
                </template>
            </span>
        </div>
        
        <div class="mt-6 mb-1 mx-auto">
            <Transition name="fade" mode="out-in">
                <FontAwesomeIcon
                    v-if="!previewData || previewData?.pending"
                    :icon="file.file_type === 'folder' ? faFolder : getFileIcon(file.name)"
                    size="9x"
                />
                <img
                    v-else
                    ref="previewImageElement"
                    :src="previewData?.value ?? ''"
                    class="bg-white -mb-5"
                    @load="(event) => {
                        const target = event.target as HTMLElement;
                        if (target.clientWidth < (containerElement!.clientWidth * 0.9)) {
                            target.classList.remove('-mb-5');
                            target.classList.add('mx-2');
                        }
                    }"
                >
            </Transition>
        </div>
    </div>
</template>