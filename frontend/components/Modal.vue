<script lang="ts">
    export default {
        inheritAttrs: false,
    };
</script>

<script setup lang="ts">
    import { faXmark } from '@fortawesome/free-solid-svg-icons';
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import { onBeforeUnmount, onMounted, ref } from 'vue';

    const emit = defineEmits<{
        close: [];
    }>();

    function close() {
        emit('close');
    }

    const dialog = ref<HTMLDialogElement>();
    const innerContainer = ref<HTMLElement>();

    async function onWindowClick(ev: MouseEvent) {
        if (ev.target instanceof HTMLElement && !innerContainer.value!.contains(ev.target)) {
            close();
        }
    }

    function onWindowKeyPress(ev: KeyboardEvent) {
        if (ev.key === 'Escape') {
            close();
        }
    }

    onMounted(async () => {
        dialog.value!.showModal();

        window.addEventListener('click', onWindowClick);
        window.addEventListener('keydown', onWindowKeyPress);
    });

    onBeforeUnmount(() => {
        window.removeEventListener('click', onWindowClick);
        window.removeEventListener('keydown', onWindowKeyPress);
    });
</script>

<template>
    <dialog
        ref="dialog"
        class="relative rounded-xl bg-black shadow-xl max-lg:!border-t-0 select-none"
    >
        <div ref="innerContainer" v-bind="$attrs">
            <slot />
        </div>

        <div class="absolute top-[0.15rem] right-1 scale-110">
            <button type="button" @click="close" class="p-1 -m-1 hover:bg-red-500 transition-colors rounded-bl-xl">
                <FontAwesomeIcon :icon="faXmark" fixed-width class="text-gray-200" />
            </button>
        </div>
    </dialog>
</template>

<style scoped>
    @keyframes dialog {
        0% {
            opacity: 0;
            transform: scale(95%);
        }

        100% {
            opacity: 100;
            transform: scale(100%);
        }
    }

    @keyframes dialog-backdrop {
        0% {
            background-color: transparent;
        }

        100% {
            background-color: rgba(31, 41, 55, 50%);
        }
    }

    dialog {
        animation: dialog ease 100ms;
    }

    dialog::backdrop {
        animation: dialog-backdrop ease 100ms;
        animation-fill-mode: forwards;
        pointer-events: none;
    }
</style>