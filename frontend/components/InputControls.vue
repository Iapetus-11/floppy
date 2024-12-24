<script setup lang="ts">
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import { faEyeSlash, faXmark } from '@fortawesome/free-solid-svg-icons';

    const container = ref<HTMLDivElement>();
    const input = ref<HTMLInputElement>();

    onMounted(() => {
        input.value = container.value!.querySelector('input') ?? undefined;

        if (!input.value) {
            throw new Error('InputControls needs to take an input in its default slot');
        }
    });

    function togglePasswordVisibility() {
        if (!input.value) {
            return;
        }

        if (input.value.type === 'password') {
            input.value.type = 'text';
        } else {
            input.value.type = 'password';
        }

        input.value.focus();
    }

    function clearInput() {
        if (input.value) {
            input.value.value = '';
            input.value.focus();
        }
    }
</script>

<template>
    <div ref="container" class="relative">
        <slot />

        <div v-if="input" class="absolute flex gap-0.5 bg-black top-1/2 right-[1px] pr-[0.5rem] pl-[0.2rem] -translate-y-1/2">
            <button
                v-if="input.type === 'password'"
                type="button"
                @click="togglePasswordVisibility"
                class="p-1 -m-1 hover:text-sky-400 transition-all hover:scale-110"
            >
                <FontAwesomeIcon :icon="faEyeSlash" size="sm" fixed-width class="scale-[73%]" />
            </button>

            <button
                type="button"
                @click="clearInput"
                class="p-1 -m-1 hover:text-red-400 transition-all hover:scale-110"
            >
                <FontAwesomeIcon :icon="faXmark" size="sm" fixed-width />
            </button>
        </div>
    </div>
</template>