<script setup lang="ts">
import { faRightFromBracket } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
</script>

<template>
    <div class="flex items-center justify-between h-[80px] mx-3">
        <h1 class="bg-teal-600 rounded-md py-1.5 px-2.5 text-3xl">Floppy</h1>

        <nav class="flex gap-2">
            <button type="button" @click="signOut" class="flex items-center button py-1.5 px-2 group">
                <div class="w-0 overflow-hidden transition-all group-hover:w-[70px]">
                    <span class=" group-hover:block whitespace-nowrap mr-2">
                        Sign Out
                    </span>
                </div>

                <FontAwesomeIcon :icon="faRightFromBracket" fixed-width size="lg" class="block" />
            </button>
        </nav>
    </div>

    <slot />
</template>