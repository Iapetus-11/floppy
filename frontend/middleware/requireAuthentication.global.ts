export default defineNuxtRouteMiddleware((to) => {
    if (to.meta.public || auth.isAuthenticated.value) {
        // Continue to destination
        return;
    }

    if (to.path !== '/login') {
        return '/login';
    }
  });