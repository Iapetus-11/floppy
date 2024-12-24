<script setup lang="ts">
    import { faSpinner } from '@fortawesome/free-solid-svg-icons';
    import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
    import { type UnwrapNestedRefs } from 'vue';
    import Alert from '~/components/Alert.vue';
    import InputControls from '~/components/InputControls.vue';

    const router = useRouter();

    definePageMeta({
        layout: 'none',
        meta: {
            public: true,
        },
    });

    const config = useRuntimeConfig();

    const formData = reactive({
        email: '',
        password: '',
    });

    const submissionState = ref<UnwrapNestedRefs<PromiseState<void>>>();

    function submit() {
        submissionState.value = reactive(usePromise<void>($fetch(
            `${config.public.apiBase}/login/email_and_password/`,
            {
                method: 'POST',
                body: JSON.stringify(formData),
                headers: { 'Content-Type': 'application/json' },
                responseType: 'json',
            },
        ).then((tokens) => {
            auth.setTokens(tokens as UserTokens);
            router.replace('/');
            return undefined;
        })));
    }

    onMounted(() => {
        if (auth.isAuthenticated.value) {
            router.replace('/');
        }
    });
</script>

<template>
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-[90vw]">
        <form @submit.prevent="submit" class="flex flex-col gap-4 max-w-96 outlined rounded-lg p-7 mx-auto">
            <h2 class="text-2xl text-center mb-4">Log In To <span class="bg-teal-600 rounded-md py-1 px-1.5 ml-0.5">Floppy</span></h2>

            <fieldset :disabled="submissionState?.pending" class="flex flex-col gap-3">
                <InputControls>
                    <input
                        v-model="formData.email"
                        type="email"
                        placeholder="email@example.com"
                        required
                        class="text-input w-full"
                    >
                </InputControls>

                <InputControls>
                    <input
                        v-model="formData.password"
                        type="password"
                        placeholder="hunter2"
                        required
                        class="text-input w-full"
                    >
                </InputControls>
            </fieldset>

            <Alert
                v-if="submissionState?.error?.response?.status === 403"
                title="Incorrect Email Or Password"
                @close="submissionState = undefined"
            />
            <Alert
                v-else-if="submissionState?.rejected"
                title="Unknown Error Occurred"
                @close="submissionState = undefined"
            >
                Please try again shortly, or contact support.
            </Alert>

            <button type="submit" :disabled="submissionState?.pending" class="button py-1.5 px-2 group">
                <span v-if="submissionState?.pending" class="block scale-125">
                    <FontAwesomeIcon :icon="faSpinner" class="animate-spin" />
                </span>
                <span v-else>Log In</span>
            </button>
        </form>
    </div>
</template>