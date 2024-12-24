<script setup lang="ts">
    import { faTriangleExclamation, faXmark } from '@fortawesome/free-solid-svg-icons';
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

    defineProps<{
        title: string,
        onClose?: () => void,
    }>();

    defineEmits<{
        close: [],
    }>();
</script>

<template>
    <div class="flex flex-col px-3.5 py-2.5 bg-red-400/90 rounded-md outlined">
        <h3 class="flex items-center w-full">
            <FontAwesomeIcon :icon="faTriangleExclamation" />
            <span class="ml-2.5">{{ title }}</span>

            <div v-if="onClose" class="ml-auto -mr-3 -mt-[1.25rem]">
                <button type="button" @click="$emit('close')" class="p-1 -m-1 hover:scale-110 hover:text-red-800 transition-all">
                    <FontAwesomeIcon :icon="faXmark" size="sm" fixed-width />
                </button>
            </div>
        </h3>

        <div class="text-sm mt-0.5">
            <slot />
        </div>
    </div>
</template>