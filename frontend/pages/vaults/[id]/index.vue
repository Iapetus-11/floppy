<script setup lang="ts">
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import type { Vault, VaultFile } from '~/models/vaults';
    import { getFileIcon } from './fileIcon';
    import { faFolder } from '@fortawesome/free-regular-svg-icons';
    import InputControls from '~/components/InputControls.vue';
    import { faUpLeft } from '@fortawesome/pro-solid-svg-icons';

    // @ts-ignore
    import pako from 'pako';

    const config = useRuntimeConfig();
    
    const router = useRouter();
    const route = useRoute();

    function encodeFolderIds(folderIds: string[]): string {
        const deflated: Uint8Array = pako.deflateRaw(folderIds.join(''));
        return btoa(String.fromCharCode(...deflated)).replaceAll('=', '').replaceAll('/', '-');
    }

    function decodeFolderIds(encodedFolderIds: string): string[] {
        const decodedb64 = atob(encodedFolderIds.replaceAll('-', '/'));
        const data = new Uint8Array([...decodedb64].map(c => c.charCodeAt(0)));
        return [...pako.inflateRaw(data, { to: 'string' }).match(/.{1,20}/g)]
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

    const filesData = await useFetch<VaultFile[]>(computed(() => {
        const url = new URL(`${config.public.apiBase}/vaults/${vault.value.id}/files`);

        if (folderId.value) {
            url.searchParams.set('parent_id', folderId.value as string);
        }

        return url.toString();
    }), {
        headers: { 'Authorization': `Bearer ${await auth.getAccessToken()}`},
        responseType: 'json',
    });

    const files = computed(() => filesData.data.value?.toSorted((a, b) => {
        if (a.file_type === b.file_type) {
            return a.name.localeCompare(b.name);
        }

        if (a.file_type === 'file') {
            return 1;
        }

        return -1;
    }));

    function handleFileClick(file: VaultFile) {
        if (file.file_type === 'folder') {
            router.push({ query: { folder: encodeFolderIds([...folderIds.value, file.id]) }});
        }
    }

    function navigateUp() {
        const remainingFolderIds = folderIds.value.slice(0, -1);
        router.push({ query: { folder: remainingFolderIds.length ? encodeFolderIds(remainingFolderIds) : undefined }})
    }
</script>

<template>
    <div class="grid grid-cols-2 gap-4">
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
                <table class="w-full" :key="folderIds.join('|')">
                    <tr
                        v-if="folderId"
                        role="button"
                        class="border-b border-t first-of-type:border-t-0 last-of-type:border-b-0 [&>td]:py-2
                               group hover:bg-teal-900/50 cursor-pointer"
                        @click="navigateUp"
                    >
                        <td class="w-0 pl-3 pr-2.5">
                            <FontAwesomeIcon
                                :icon="faUpLeft"
                                class="text-gray-400 group-hover:text-gray-200"
                            />
                        </td>

                        <td class="text-gray-300 group-hover:text-gray-200 whitespace-nowrap">..</td>
                    </tr>

                    <tr
                        v-for="file in files"
                        role="button"
                        class="border-b border-t first-of-type:border-t-0 last-of-type:border-b-0 [&>td]:py-2
                               group hover:bg-teal-900/50 cursor-pointer"
                        @mouseenter="previewedFile = file"
                        @mouseleave="previewedFile.id === file.id ? previewedFile = undefined : {}"
                        @click="handleFileClick(file)"
                    >
                        <td class="w-0 pl-3 pr-2.5">
                            <FontAwesomeIcon
                                :icon="file.file_type === 'folder' ? faFolder : getFileIcon(file.name)"
                                class="text-gray-400 group-hover:text-gray-200"
                            />
                        </td>

                        <td class="text-gray-300 group-hover:text-gray-200 whitespace-nowrap text-ellipsis">{{ file.name }}</td>
                    </tr>

                    <tr
                        v-if="!files?.length"
                    >
                        <td></td>
                        <td class="pt-3 italic text-gray-300/90">There's nothing here... :/</td>
                    </tr>
                </table>
            </Transition>
        </div>

        <div class="p-2 outlined h-fit">
            <div v-if="previewedFile" class="flex flex-col items-center py-4">
                <div class="flex flex-col items-center w-full">
                    <h2 class="text-xl font-semibold text-center w-[95%] break-all">
                        {{ previewedFile.name }}
                    </h2>
                    <span class="text-center">
                        <template v-if="previewedFile.created_at !== null">
                            Created {{ formatDuration((Number(new Date()) - Number(new Date(previewedFile.created_at))) / 1000) }} ago
                        </template>

                        <template v-if="previewedFile.size !== null && previewedFile.file_type === 'file'">
                            &middot; {{ formatFileSize(previewedFile.size) }}
                        </template>
                    </span>
                </div>
                
                <div class="mt-6 mb-1 mx-auto">
                    <FontAwesomeIcon
                        :icon="previewedFile.file_type === 'folder' ? faFolder : getFileIcon(previewedFile.name)"
                        size="9x"
                    />
                </div>
            </div>
            <p v-else>
                Hover over a file to preview it...
            </p>
        </div>
    </div>


    <!-- <code class="block py-2 px-3 outlined font-mono max-w-[80vw] mx-auto mt-20 overflow-hidden">
        Vault: {{ vault }}
        <br><br>
        Files: {{ files }}
    </code> -->
</template>
