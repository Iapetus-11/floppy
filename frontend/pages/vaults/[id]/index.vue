<script setup lang="ts">
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import type { Vault, VaultFile } from '~/models/vaults';
    import { faFolder } from '@fortawesome/free-regular-svg-icons';
    import InputControls from '~/components/InputControls.vue';
    import { faUpLeft } from '@fortawesome/pro-solid-svg-icons';
    import FilePreview from '~/components/FilePreview.vue';
    import { faRectangleTerminal } from '@fortawesome/pro-regular-svg-icons';
    import CopyDownloadCommandModal from '~/components/CopyDownloadCommandModal.vue';

    // @ts-ignore
    import pako from 'pako';

    const config = useRuntimeConfig();
    
    const route = useRoute();

    function encodeFolderIds(folderIds: string[]): string {
        const deflated: Uint8Array = pako.deflateRaw(folderIds.join(''));
        return btoa(String.fromCharCode(...deflated)).replaceAll('=', '').replaceAll('/', '-');
    }

    function decodeFolderIds(encodedFolderIds: string): string[] {
        const decodedb64 = atob(encodedFolderIds.replaceAll('-', '/'));
        const data = new Uint8Array([...decodedb64].map(c => c.charCodeAt(0)));
        return [...(pako.inflateRaw(data, { to: 'string' }).match(/.{1,20}/g) ?? [])];
    }

    const folderIds = computed(() => {
        if (typeof route.query.folder === 'string') {
            return decodeFolderIds(route.query.folder);
        }

        return [];
    });
    const folderId = computed(() => folderIds.value.at(-1));

    const vaults = inject('vaults') as Ref<Map<string, Vault>>;
    const vault = computed(() => vaults.value.get(route.params.id as string)!);

    const searchQuery = ref<string>('');
    const previewedFile = ref<VaultFile>();

    const filesRequest = await useFetch<VaultFile[]>(computed(() => {
        const url = new URL(`${config.public.apiBase}/vaults/${vault.value.id}/files`);

        if (folderId.value) {
            url.searchParams.set('parent_id', folderId.value as string);
        }

        return url.toString();
    }), {
        query: { search: searchQuery },
        headers: { 'Authorization': `Bearer ${await auth.getAccessToken()}` },
        responseType: 'json',
    });

    const files = computed(() => filesRequest.data.value?.toSorted((a, b) => {
        if (a.file_type === b.file_type) {
            return a.name.localeCompare(b.name);
        }

        if (a.file_type === 'file') {
            return 1;
        }

        return -1;
    }));

    async function handleFileClick(file: VaultFile) {
        if (file.file_type === 'file') {
            downloadFile(
                `${config.public.apiBase}/vaults/${vault.value.id}/files/${file.id}`,
                file.name,
                { headers: { 'Authorization': `Bearer ${await auth.getAccessToken()}` },
            });
        }
    }

    const navigateUpQueryString = computed<string>(() => {
        const remainingFolderIds = folderIds.value.slice(0, -1);
        return remainingFolderIds.length ? `?folder=${encodeFolderIds(remainingFolderIds)}` : '?';
    });

    const copyDownloadCommandModalFile = ref<VaultFile>();
</script>

<template>
    <div class="grid grid-cols-2 gap-4 mb-6">
        <div>
            <div class="mb-3">
                <InputControls>
                    <input
                        v-model="searchQuery"
                        type="search"
                        class="text-input w-full"
                        placeholder="Search..."
                        data-focus-ring-off
                    >
                </InputControls>
            </div>

            <Transition mode="out-in" name="fade" :duration="100">
                <table class="w-full table-fixed overflow-hidden" :key="folderIds.join('|')">
                    <tr
                        v-if="folderId"
                        role="button"
                        class="relative border-b border-t first-of-type:border-t-0 last-of-type:border-b-0 [&>td]:py-2
                               group hover:bg-teal-700/80 cursor-pointer transition-all"
                    >
                        <td class="w-0 pl-3 pr-2.5">
                            <FontAwesomeIcon
                                :icon="faUpLeft"
                                class="text-gray-400 group-hover:text-gray-200"
                            />
                        </td>

                        <td class="text-gray-300 group-hover:text-gray-200 whitespace-nowrap pl-2.5">
                            ..
                            &ThickSpace;
                            <div class="inline-flex whitespace-nowrap overflow-hidden w-[150%] justify-end">
                                {{ files?.[0]?.path_id.split('/').slice(0, -1).join('/') }}
                            </div>

                            <NuxtLink
                                :href="navigateUpQueryString"
                                class="opacity-0 absolute top-0 left-0 w-full h-full"
                            >
                            </NuxtLink>
                        </td>

                        <td></td>
                    </tr>

                    <tr
                        v-for="file in files"
                        :key="file.id"
                        role="button"
                        class="relative border-b border-t first-of-type:border-t-0 last-of-type:border-b-0 [&>td]:py-2
                               group hover:bg-teal-600/80 cursor-pointer w-full transition-all"
                        @mouseenter="previewedFile = file"
                        @click="handleFileClick(file)"
                    >
                        <td class="w-0 pl-3 pr-2.5">
                            <FontAwesomeIcon
                                :icon="file.file_type === 'folder' ? faFolder : getFileIcon(file.name)"
                                class="text-gray-400 group-hover:text-gray-200"
                            />
                        </td>

                        <td class="text-gray-300 group-hover:text-gray-200 whitespace-nowrap text-ellipsis pl-2.5 pr-2.5 overflow-hidden w-[calc(100%_-_50px)]">
                            {{ file.name }}

                            <NuxtLink
                                v-if="file.file_type === 'folder'"
                                @click.stop
                                :href="`?folder=${encodeFolderIds([...folderIds, file.id])}`"
                                class="opacity-0 absolute top-0 left-0 w-full h-full"
                            >
                            </NuxtLink>
                        </td>

                        <td v-if="file.file_type === 'file'" class="w-[50px]">
                            <div class="flex justify-end px-1">
                                <button
                                    @click.stop="copyDownloadCommandModalFile = file"
                                    type="button"
                                    class="text-gray-400 group-hover:text-gray-200 hover:bg-teal-600 
                                             transition-colors px-1 py-2 -mx-1 -my-2 pr-2 -mr-2 rounded-l-md"
                                >
                                    <FontAwesomeIcon :icon="faRectangleTerminal" fixed-width />
                                </button>
                            </div>
                        </td>
                        <td v-else class="w-[50px]"></td>
                    </tr>

                    <tr v-if="!files?.length">
                        <td></td>
                        <td class="pt-3 italic text-gray-300/90">There's nothing here... :/</td>
                    </tr>
                </table>
            </Transition>
        </div>

        <FilePreview
            v-if="previewedFile"
            :file="previewedFile"
            :vault-id="vault.id"
        />
        <p v-else class="p-2 outlined h-fit">
            Hover over a file to preview it...
        </p>
    </div>

    <CopyDownloadCommandModal
        v-if="copyDownloadCommandModalFile"
        @close="copyDownloadCommandModalFile = undefined"
        :file="copyDownloadCommandModalFile"
    />
</template>
