<script setup lang="ts">
    import { faSpinner } from '@fortawesome/free-solid-svg-icons';
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import type { Vault } from '~/models/vaults';

    const router = useRouter();

    watch(inject('vaults') as Ref<Map<string, Vault>>, (vaults) => {
        const firstVault = vaults.values().next().value;

        if (!firstVault) {
            return;
        }

        router.push(`/vaults/${firstVault.id}`);
    }, { immediate: true });
</script>

<template>
    <!-- TODO: Replace with skeleton loading -->
    <div class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
        <FontAwesomeIcon :icon="faSpinner" size="4x" class="animate-spin" />
    </div>
</template>