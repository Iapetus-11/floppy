import type { UnwrapNestedRefs } from "vue";
import type { PromiseState } from "./promises";

export type UserTokens = {
    access_token: string,
    refresh_token: string,
};

export function signOut() {
    localStorage.removeItem('auth__access_token');
    localStorage.removeItem('auth__refresh_token');
    window.location.reload();
}

function createAuth() {
    const accessToken = ref();
    const refreshToken = ref();

    const isAuthenticated = computed(() => !!refreshToken.value);
    const tokenData = computed<Record<string, any>>(() => {
        const b64Data = accessToken.value?.split('.')[1]?.replace(/-/g, '+').replace(/_/g, '/');
        return JSON.parse(decodeURIComponent(
            atob(b64Data)
            .split('')
            .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
            .join('')
        ));
    });

    // --------------------------------------------------------------------------------
    
    function setTokens(tokens: Partial<UserTokens>) {
        accessToken.value = tokens.access_token;
        refreshToken.value = tokens.refresh_token;

        if (tokens.access_token) {
            localStorage.setItem('auth__access_token', tokens.access_token);
        }

        if (tokens.refresh_token) {
            localStorage.setItem('auth__refresh_token', tokens.refresh_token);
        }
    }

    function loadTokens() {
        accessToken.value = localStorage.getItem('auth__access_token') || undefined;
        refreshToken.value = localStorage.getItem('auth__refresh_token') || undefined;
    }

    // --------------------------------------------------------------------------------

    function isAccessTokenExpired(): boolean {
        return (Date.now() / 1000) > tokenData.value.exp;
    }

    const refreshState = ref<UnwrapNestedRefs<PromiseState<void>>>();
    function performRefreshIfNeeded(): Promise<void> {
        if (!refreshToken.value) {
            console.log('inside performRefreshIfNeeded - !refreshToken.value');
            return Promise.resolve();
        }

        if (!isAccessTokenExpired()) {
            console.log('inside performRefreshIfNeeded - !isAccessTokenExpired()');
            return Promise.resolve();
        }

        console.log("setting refreshState.value");

        refreshState.value = reactive(usePromise<void>(
            $fetch(`${import.meta.env.BASE_URL}/tokens/refresh/`, {
                method: 'POST',
                body: JSON.stringify({ refresh_token: refreshToken.value }),
                headers: { 'Content-Type': 'application/json' },
                responseType: 'json',
            }).then((data) => void setTokens(data as { access_token: string })),
        ));

        return refreshState.value.promise;
    }

    function getAccessToken(): Promise<string> {
        console.log('Getting access token...');

        if (!isAuthenticated.value) {
            console.log('nav to login screen b/c !isAuthenticated.value from getAccessToken');
            window.location.replace('/login');
            return new Promise(() => {});
        }

        return performRefreshIfNeeded().then(() => accessToken.value);
    }

    // --------------------------------------------------------------------------------

    function handleStorageEvent(ev: StorageEvent) {
        if (ev.key?.startsWith('auth__')) {
            console.log('handleStorageEvent called');
            loadTokens();
        }
    }

    window.addEventListener('storage', handleStorageEvent);
    loadTokens();
    performRefreshIfNeeded();

    // --------------------------------------------------------------------------------

    return {
        tokenData,
        isAuthenticated,
        setTokens,
        getAccessToken,
    }
}

export default createAuth();