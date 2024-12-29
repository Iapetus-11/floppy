<script setup lang="ts">
    import { faRightFromBracket } from '@fortawesome/free-solid-svg-icons';
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import type { Vault } from '~/models/vaults';

    const config = useRuntimeConfig();

    const route = useRoute();

    const vaults = await useFetch<Vault[]>(`${config.public.apiBase}/vaults`, {
        headers: { 'Authorization': `Bearer ${await auth.getAccessToken()}`},
        responseType: 'json',
    });

    provide('vaults', computed(() => {
        const vaultsData = vaults.data.value;
        const vaultsMapping = new Map<string, Vault>();

        if (vaultsData) {
            for (const vault of vaultsData) {
                vaultsMapping.set(vault.id, vault);
            }
        }

        return vaultsMapping;
    }));
</script>

<template>
    <div class="mx-auto max-w-[80rem] px-4">
        <div class="flex items-center justify-between h-[80px]">
            <h1 class="bg-teal-600 rounded-md py-1.5 px-2.5 text-3xl font-medium">Floppy</h1>

            <nav class="flex gap-2">
                <NuxtLink
                    v-for="vault in vaults.data.value"
                    :key="vault.id"
                    :href="`/vaults/${vault.id}`"
                    class="outlined px-3 py-2 link-button"
                    :class="{ 'border-teal-300': route.path === `/vaults/${vault.id}` }"
                >
                    {{ vault.name }}
                </NuxtLink>

                <button type="button" @click="signOut" class="flex items-center button py-1.5 px-2 group">
                    <div class="w-0 overflow-hidden transition-all group-hover:w-[70px]">
                        <span class="group-hover:block whitespace-nowrap mr-2">
                            Sign Out
                        </span>
                    </div>

                    <FontAwesomeIcon :icon="faRightFromBracket" fixed-width size="lg" class="block" />
                </button>
            </nav>
        </div>

        <slot />
    </div>
</template>