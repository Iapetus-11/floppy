export interface PromiseState<T, E = any> {
    value: Ref<T | undefined>;
    error: Ref<E | undefined>;
    pending: Ref<boolean>;
    fulfilled: Ref<boolean>;
    rejected: Ref<boolean>;
    promise: Promise<T>;
}

/**
 * Populated if there was an error handler but an error was unhandled and sent to Sentry
 */
export const PROMISE_UNHANDLED_ERROR = Symbol('UNHANDLED');

/**
 * Create reactive state for a promise, optionally with a special error handler
 * @returns Refs relating to the state of the promise
 */
export function usePromise<T, E = any>(
    promise: Promise<T>,
    errorHandler?: (error: any) => E | undefined,
): PromiseState<T, E | typeof PROMISE_UNHANDLED_ERROR> {
    promise = Promise.resolve(promise); // Make sure we're dealing with a promise

    function handleError(error: any) {
        if (!errorHandler) {
            console.error(error);
            return error;
        }

        const errorHandlerReturn = errorHandler(error);

        if (errorHandlerReturn === undefined) {
            console.error(error);
            return PROMISE_UNHANDLED_ERROR;
        }

        return errorHandlerReturn;
    }

    const state: PromiseState<T> = {
        value: ref(undefined),
        error: ref(undefined),
        pending: ref(true),
        fulfilled: ref(false),
        rejected: ref(false),
        promise,
    };

    promise.then(
        (value) => {
            state.value.value = value;
            state.pending.value = false;
            state.fulfilled.value = true;
        },
        (error) => {
            state.error.value = handleError(error);
            state.pending.value = false;
            state.rejected.value = true;
        },
    );

    return state;
}