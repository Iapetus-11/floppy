<script setup lang="ts">
    import type { VaultFile } from '~/models/vaults';
    import Modal from './Modal.vue';

    const config = useRuntimeConfig();

    const props = defineProps<{
        file: VaultFile,
    }>();

    defineEmits<{
        close: [],
    }>();

    const fileAccessCodeRequest = await useFetch<{ code: string }>(computed(() => 
        `${config.public.apiBase}/vaults/${props.file.vault_id}/files/${props.file.id}/access_code`,
    ), {
        headers: { 'Authorization': `Bearer ${await auth.getAccessToken()}` },
        responseType: 'json',
    });

    const fileDownloadUrl = computed<string>(() => {
        const url = new URL(`${config.public.apiBase}/vaults/${props.file.vault_id}/files/${props.file.id}`);
        url.searchParams.set('code', fileAccessCodeRequest.data.value?.code ?? '');
        console.log(url);
        return url.toString();
    });

    function getEscapedFileName() {
        return props.file.name.replaceAll('"', '').replaceAll("'", '').replaceAll('\\', '_').replaceAll('/', '_');
    }

    const curlCommand = computed(() => `curl -o "${getEscapedFileName()}" ${fileDownloadUrl.value}`);
    const powershellCommand = computed(() => `Invoke-WebRequest ${fileDownloadUrl.value} -OutFile ".\\${getEscapedFileName()}"`);
</script>

<template>
    <Modal @close="$emit('close')" class="flex flex-col w-[89vw] md:w-[32rem] gap-3 p-4">
        <h2 class="text-gray-200 text-2xl sm:text-center">Copy Download Command</h2>

        <div>
            <label for="curl-command-textarea" class="text-gray-200 text-xs ml-0.5">MacOS / Linux</label>
            
            <div class="relative">
                <textarea
                    id="curl-command-textarea"
                    disabled
                    :value="curlCommand"
                    class="text-input font-mono text-xs w-full resize-none cursor-copy overflow-y-scroll h-[6.25rem] md:h-22"
                >
                </textarea>

                <button
                    type="button"
                    @click="copyText(curlCommand)"
                    class="absolute opacity-0 top-[0.25px] left-[0.25px] w-[calc(100%_-_0.75px)] hover:opacity-80 
                         bg-teal-600 h-[calc(6.25rem_-_0.75px)] rounded-md tracking-wider font-medium text-xl 
                         text-white transition-all"
                >
                    Click To Copy
                </button>
            </div>
        </div>

        <div>
            <label for="powershell-command-textarea" class="text-gray-200 text-xs ml-0.5">Powershell</label>

            <div class="relative">
                <textarea
                    id="powershell-command-textarea"
                    disabled
                    :value="powershellCommand"
                    class="text-input font-mono text-xs w-full resize-none cursor-copy overflow-y-scroll h-[6.25rem] md:h-22"
                >
                </textarea>

                <button
                    type="button"
                    @click="copyText(powershellCommand)"
                    class="absolute opacity-0 top-[0.25px] left-[0.25px] w-[calc(100%_-_0.75px)] hover:opacity-80 
                          bg-teal-600 h-[calc(6.25rem_-_0.75px)] rounded-md tracking-wide font-medium text-xl 
                          text-white transition-all"
                >
                    Click To Copy
                </button>
            </div>
        </div>
    </Modal>
</template>